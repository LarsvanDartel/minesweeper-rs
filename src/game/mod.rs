mod board;
mod options;
mod tilemap;

use std::collections::VecDeque;

use crate::{
    style::{game_assets::GameAssets, ui_assets::UiAssets},
    util::despawn_all,
    AppState,
};

use board::Board;

#[cfg(feature = "debug")]
use bevy::log;

use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState},
    prelude::*,
    utils::HashSet,
};
use options::GameOptions;

use self::{
    options::TileSize,
    tilemap::{TileMap, TileType},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum GameState {
    Options,
    Playing,
    Paused,
    Finished,
    #[default]
    Inactive,
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct OnPauseScreen;

#[derive(Component)]
struct OnFinishedScreen;

#[derive(Component)]
struct Position(UVec2);

#[derive(Component)]
struct Tile(TileType);

#[derive(Component)]
struct Cover;

#[derive(Component)]
struct Flag;

#[derive(Component)]
enum OverlayButtonAction {
    Restart,
    ReturnToMenu,
    Continue,
}

#[derive(Event)]
pub struct TileRevealed {
    pub position: UVec2,
}

#[derive(Event)]
pub struct TileFlagged {
    pub position: UVec2,
}

#[derive(Resource)]
struct GameResult(bool);

// Constants for the z-index of the various game objects
/// The z-index of the background
const BACKGROUND_Z: f32 = 0.;

/// The z-index of the tiles
const TILE_Z: f32 = 1.;

/// The z-index of the text displaying
/// the number of bombs around a tile
const BOMB_COUNT_Z: f32 = 2.;

/// The z-index of the cover
const COVER_Z: f32 = 3.;

/// The z-index of the flag sprite
const FLAG_Z: f32 = 4.;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_event::<TileRevealed>()
            .add_event::<TileFlagged>()
            .add_plugins(options::GameOptionsPlugin)
            .add_systems(OnEnter(AppState::Game), Self::start_setup)
            .add_systems(OnExit(AppState::Game), despawn_all::<OnGameScreen>)
            .add_systems(OnEnter(GameState::Playing), Self::start_game)
            .add_systems(OnEnter(GameState::Paused), Self::pause)
            .add_systems(OnExit(GameState::Paused), despawn_all::<OnPauseScreen>)
            .add_systems(OnEnter(GameState::Finished), Self::game_finished)
            .add_systems(
                OnExit(GameState::Finished),
                (despawn_all::<OnGameScreen>, despawn_all::<OnFinishedScreen>),
            )
            .add_systems(
                Update,
                (
                    Self::handle_keyboard_input,
                    Self::handle_mouse_input,
                    Self::handle_reveal_event,
                    Self::handle_flag_event,
                    Self::check_finished,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    (Self::overlay_button_color, Self::button_actions)
                        .run_if(in_state(GameState::Paused)),
                    (Self::overlay_button_color, Self::button_actions)
                        .run_if(in_state(GameState::Finished)),
                ),
            );
    }
}

impl GamePlugin {
    fn start_setup(mut game_state: ResMut<NextState<GameState>>, board: Option<Res<Board>>) {
        if board.is_some() {
            game_state.set(GameState::Playing);
        } else {
            game_state.set(GameState::Options);
        }
    }

    fn handle_keyboard_input(
        mut keyboard_evr: EventReader<KeyboardInput>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        for event in keyboard_evr.read() {
            if event.key_code == KeyCode::KeyP {
                game_state.set(GameState::Paused);
            }
        }
    }

    fn handle_mouse_input(
        window: Query<&Window>,
        board: Res<Board>,
        mut mouse_button_evr: EventReader<MouseButtonInput>,
        mut tile_revealed_evw: EventWriter<TileRevealed>,
        mut tile_flagged_evw: EventWriter<TileFlagged>,
    ) {
        let window = &window.single();

        for event in mouse_button_evr.read() {
            match event.state {
                ButtonState::Pressed => {
                    if let Some(cursor_position) = window.cursor_position() {
                        if let Some(position) = board.mouse_to_tile(window, cursor_position) {
                            if event.button == MouseButton::Left {
                                tile_revealed_evw.send(TileRevealed { position });
                            } else if event.button == MouseButton::Right {
                                tile_flagged_evw.send(TileFlagged { position });
                            }
                        }
                    }
                }
                ButtonState::Released => {}
            }
        }
    }

    fn handle_reveal_event(
        mut commands: Commands,
        mut board: ResMut<Board>,
        mut tile_revealed_evr: EventReader<TileRevealed>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        let mut queue = VecDeque::new();
        for event in tile_revealed_evr.read() {
            let tile = match board.tile_map.get_tile(event.position) {
                Some(tile) => tile,
                None => {
                    #[cfg(feature = "debug")]
                    log::error!("Could not find tile for position {}", event.position);
                    continue;
                }
            };
            if tile.cover.is_none() {
                if let TileType::Number(count) = tile.tile_type {
                    if count
                        == board
                            .tile_map
                            .get_neighbors(event.position)
                            .filter(|pos| board.tile_map.get_tile(*pos).unwrap().flag.is_some())
                            .count()
                    {
                        for neighbor in board.tile_map.get_neighbors(event.position) {
                            queue.push_back(neighbor);
                        }
                    }
                }
            } else {
                queue.push_back(event.position);
            }
        }

        let mut revealed = HashSet::new();

        while let Some(position) = queue.pop_front() {
            if !revealed.insert(position) {
                continue;
            }

            let tile = match board.tile_map.get_tile_mut(position) {
                Some(tile) => tile,
                None => {
                    #[cfg(feature = "debug")]
                    log::error!("Could not find tile for position {}", position);
                    continue;
                }
            };

            if tile.flag.is_some() {
                continue;
            }

            if let Some(cover_entity) = tile.cover.take() {
                commands.entity(cover_entity).despawn_recursive();
            } else {
                continue;
            }

            match tile.tile_type {
                TileType::Bomb => {
                    commands.insert_resource(GameResult(false));
                    game_state.set(GameState::Finished);
                }
                TileType::Empty => {
                    for neighbor in board.tile_map.get_neighbors(position) {
                        queue.push_back(neighbor);
                    }
                }
                TileType::Number(_) => {}
            }
        }
    }

    fn handle_flag_event(
        mut commands: Commands,
        mut tile_flagged_evr: EventReader<TileFlagged>,
        mut board: ResMut<Board>,
        game_assets: Res<GameAssets>,
    ) {
        for event in tile_flagged_evr.read() {
            let tile_size = board.tile_size;

            let tile = match board.tile_map.get_tile_mut(event.position) {
                Some(tile) => tile,
                None => {
                    #[cfg(feature = "debug")]
                    log::error!("Could not find tile for position {}", event.position);
                    continue;
                }
            };

            if tile.cover.is_none() {
                continue;
            }

            if let Some(flag_entity) = tile.flag.take() {
                commands.entity(flag_entity).despawn_recursive();
            } else {
                let custom_size = Some(Vec2::splat(tile_size));
                let flag_entity = commands
                    .spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size,
                                color: game_assets.tile_flagged,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., FLAG_Z),
                            ..Default::default()
                        },
                        Position(event.position),
                        Flag,
                    ))
                    .id();

                commands
                    .entity(tile.entity.unwrap())
                    .push_children(&[flag_entity]);
                tile.flag = Some(flag_entity);
            }
        }
    }

    fn check_finished(
        mut commands: Commands,
        board: Res<Board>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        if board.all_revealed() {
            commands.insert_resource(GameResult(true));
            game_state.set(GameState::Finished);
        }
    }

    fn start_game(
        mut commands: Commands,
        window: Query<&Window>,
        board: Option<Res<Board>>,
        game_options: Res<GameOptions>,
        game_assets: Res<GameAssets>,
    ) {
        if board.is_some() {
            return;
        }

        let tile_size = match game_options.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { min, max } => {
                let window = &window.single();
                let tile_width = window.width() / game_options.size.x as f32;
                let tile_height = window.height() / game_options.size.y as f32;

                (tile_width.min(tile_height) - game_options.tile_padding).clamp(min, max)
            }
        };

        let mut tile_map = TileMap::empty(game_options.size);
        tile_map.set_bombs(game_options.bomb_count);

        #[cfg(feature = "debug")]
        log::info!("{:?}", tile_map);

        let board_size = tile_map.size().as_vec2() * (tile_size + game_options.tile_padding)
            - game_options.tile_padding;
        let board_position = Vec3::new(-board_size.x / 2., -board_size.y / 2., BACKGROUND_Z);

        let board_entity = commands
            .spawn((
                Name::new("Board"),
                SpatialBundle {
                    transform: Transform::from_translation(board_position),
                    ..Default::default()
                },
                OnGameScreen,
            ))
            .with_children(|parent| {
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: game_assets.board,
                        custom_size: Some(board_size),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        board_size.x / 2.,
                        board_size.y / 2.,
                        BACKGROUND_Z,
                    ),
                    ..Default::default()
                });
            })
            .id();

        let size = game_options.size;
        let tile_padding = game_options.tile_padding;
        let custom_size = Some(Vec2::splat(tile_size));

        let cover = SpriteBundle {
            sprite: Sprite {
                custom_size,
                color: game_assets.tile_covered,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., COVER_Z),
            ..Default::default()
        };

        for y in 0..size.y {
            for x in 0..size.x {
                let position = UVec2::new(x, y);
                let tile = tile_map.get_tile_mut(position).unwrap();

                let sprite = SpriteBundle {
                    sprite: Sprite {
                        color: game_assets.tile_uncovered,
                        custom_size,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        x as f32 * (tile_size + tile_padding) + (tile_size / 2.),
                        (size.y - y - 1) as f32 * (tile_size + tile_padding) + (tile_size / 2.),
                        TILE_Z,
                    ),
                    ..Default::default()
                };

                let tile_entity = commands
                    .spawn((sprite, Position(position), Tile(tile.tile_type)))
                    .id();

                let cover_entity = commands
                    .spawn((cover.clone(), Position(position), Cover))
                    .id();

                tile.entity = Some(tile_entity);
                tile.cover = Some(cover_entity);

                let mut children = vec![cover_entity];

                match tile.tile_type {
                    TileType::Bomb => {
                        children.push(
                            commands
                                .spawn(SpriteBundle {
                                    sprite: Sprite {
                                        custom_size,
                                        color: game_assets.tile_mine,
                                        ..Default::default()
                                    },
                                    transform: Transform::from_xyz(0., 0., BOMB_COUNT_Z),
                                    ..Default::default()
                                })
                                .id(),
                        );
                    }
                    TileType::Number(count) => {
                        children.push(
                            commands
                                .spawn(Text2dBundle {
                                    text: Text::from_section(
                                        count.to_string(),
                                        TextStyle {
                                            font: game_assets.tile_count_font.clone(),
                                            font_size: tile_size,
                                            color: game_assets.count_color(count),
                                        },
                                    ),
                                    transform: Transform::from_xyz(0., 0., BOMB_COUNT_Z),
                                    ..Default::default()
                                })
                                .id(),
                        );
                    }
                    TileType::Empty => {}
                }

                commands.entity(tile_entity).push_children(&children);
                commands.entity(board_entity).push_children(&[tile_entity]);
            }
        }

        if game_options.safe_start {
            let position = tile_map.find_empty_tile().unwrap();
            let tile = tile_map.get_tile_mut(position).unwrap();

            let new_cover = commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size,
                            color: game_assets.tile_uncovered,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0., 0., COVER_Z),
                        ..Default::default()
                    },
                    Position(position),
                    Cover,
                ))
                .id();

            let old_cover = tile.cover.replace(new_cover).unwrap();
            commands.entity(old_cover).despawn_recursive();
            commands
                .entity(tile.entity.unwrap())
                .push_children(&[new_cover]);
        }

        commands.insert_resource(Board {
            tile_map,
            position: board_position.xy(),
            size: board_size,
            tile_size,
            tile_padding,
        });
    }

    fn overlay_button_color(
        mut interaction_query: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
        ui_assets: Res<UiAssets>,
    ) {
        for (interaction, mut color) in interaction_query.iter_mut() {
            *color = match interaction {
                Interaction::Pressed => ui_assets.accent.into(),
                Interaction::Hovered => ui_assets.accent_alt.into(),
                Interaction::None => ui_assets.background_alt.into(),
            };
        }
    }

    fn button_actions(
        mut commands: Commands,
        interaction_query: Query<(&Interaction, &OverlayButtonAction), Changed<Interaction>>,
        mut app_state: ResMut<NextState<AppState>>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        for (interaction, action) in interaction_query.iter() {
            if *interaction != Interaction::Pressed {
                return;
            }
            match action {
                OverlayButtonAction::Restart => {
                    commands.remove_resource::<Board>();
                    game_state.set(GameState::Playing);
                }
                OverlayButtonAction::ReturnToMenu => {
                    // TODO: Find a way to keep current game
                    commands.remove_resource::<Board>();
                    game_state.set(GameState::Inactive);
                    app_state.set(AppState::Menu);
                }
                OverlayButtonAction::Continue => {
                    game_state.set(GameState::Playing);
                }
            }
        }
    }
    fn pause(mut commands: Commands, ui_assets: Res<UiAssets>) {
        let background_color = Color::rgba(
            ui_assets.background.r(),
            ui_assets.background.g(),
            ui_assets.background.b(),
            0.4,
        )
        .into();

        let overlay = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color,
                    ..Default::default()
                },
                OnPauseScreen,
            ))
            .id();

        let column = commands
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();

        let pause_text = commands
            .spawn(TextBundle::from_section(
                "Paused",
                TextStyle {
                    font: ui_assets.font.clone(),
                    font_size: 80.,
                    color: ui_assets.foreground,
                },
            ))
            .id();

        let button_style = Style {
            width: Val::Px(250.),
            height: Val::Px(65.),
            margin: UiRect::all(Val::Px(20.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        };

        let button_text_style = TextStyle {
            font_size: 40.,
            color: ui_assets.foreground,
            font: ui_assets.font.clone(),
        };

        let continue_button = commands
            .spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: ui_assets.background_alt.into(),
                    ..Default::default()
                },
                OverlayButtonAction::Continue,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Continue",
                    button_text_style.clone(),
                ));
            })
            .id();

        let return_to_menu_button = commands
            .spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: ui_assets.background_alt.into(),
                    ..Default::default()
                },
                OverlayButtonAction::ReturnToMenu,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section("Menu", button_text_style.clone()));
            })
            .id();

        commands.entity(overlay).push_children(&[column]);
        commands.entity(column).push_children(&[
            pause_text,
            continue_button,
            return_to_menu_button,
        ]);
    }

    fn game_finished(
        mut commands: Commands,
        game_result: Res<GameResult>,
        mut board: ResMut<Board>,
        ui_assets: Res<UiAssets>,
    ) {
        for tile in board.tile_map.iter_mut() {
            if tile.is_bomb() {
                if let Some(cover_entity) = tile.cover.take() {
                    commands.entity(cover_entity).despawn_recursive();
                }
            }
        }

        let result_color = if game_result.0 {
            Color::GREEN
        } else {
            Color::RED
        };

        // Make color transparent
        let background_color =
            Color::rgba(result_color.r(), result_color.g(), result_color.b(), 0.1).into();

        let finished_screen = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color,
                    ..Default::default()
                },
                OnFinishedScreen,
            ))
            .id();

        let column = commands
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();

        let result_text = if game_result.0 {
            "You win!"
        } else {
            "You lose!"
        };

        let text_entity = commands
            .spawn(TextBundle::from_section(
                result_text,
                TextStyle {
                    font: ui_assets.font.clone(),
                    font_size: 80.,
                    color: result_color,
                },
            ))
            .id();

        let button_style = Style {
            width: Val::Px(250.),
            height: Val::Px(65.),
            margin: UiRect::all(Val::Px(20.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        };

        let button_text_style = TextStyle {
            font_size: 40.,
            color: ui_assets.foreground,
            font: ui_assets.font.clone(),
        };

        let restart_button = commands
            .spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: ui_assets.background_alt.into(),
                    ..Default::default()
                },
                OverlayButtonAction::Restart,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Restart",
                    button_text_style.clone(),
                ));
            })
            .id();

        let return_to_menu_button = commands
            .spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: ui_assets.background_alt.into(),
                    ..Default::default()
                },
                OverlayButtonAction::ReturnToMenu,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section("Menu", button_text_style.clone()));
            })
            .id();

        commands.entity(finished_screen).push_children(&[column]);

        commands.entity(column).push_children(&[
            text_entity,
            restart_button,
            return_to_menu_button,
        ]);
    }
}
