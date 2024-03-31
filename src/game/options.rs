use bevy::prelude::*;

use crate::{style::ui_assets::UiAssets, util::despawn_all, AppState};

use super::{board::Board, GameState};

#[derive(Clone, Resource)]
pub struct GameOptions {
    pub size: UVec2,
    pub bomb_count: u32,
    pub safe_start: bool,
    pub tile_size: TileSize,
    pub tile_padding: f32,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            size: Preset::Beginner.size(),
            bomb_count: Preset::Beginner.bomb_count(),
            safe_start: true,
            tile_size: TileSize::default(),
            tile_padding: 2.,
        }
    }
}

#[derive(Clone)]
pub enum TileSize {
    Fixed(f32),
    Adaptive { min: f32, max: f32 },
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive { min: 10., max: 50. }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Preset {
    Beginner,
    Intermediate,
    Expert,
}

impl Preset {
    pub fn values() -> impl Iterator<Item = Preset> {
        [Preset::Beginner, Preset::Intermediate, Preset::Expert]
            .iter()
            .copied()
    }

    fn size(&self) -> UVec2 {
        match self {
            Preset::Beginner => (9, 9).into(),
            Preset::Intermediate => (16, 16).into(),
            Preset::Expert => (30, 16).into(),
        }
    }

    fn bomb_count(&self) -> u32 {
        match self {
            Preset::Beginner => 10,
            Preset::Intermediate => 40,
            Preset::Expert => 99,
        }
    }
}

impl ToString for Preset {
    fn to_string(&self) -> String {
        match self {
            Preset::Beginner => "Beginner".to_string(),
            Preset::Intermediate => "Intermediate".to_string(),
            Preset::Expert => "Expert".to_string(),
        }
    }
}

#[derive(Component)]
struct OnOptionsScreen;

#[derive(Component)]
enum SettingsTextField {
    Rows,
    Columns,
    BombCount,
}

#[derive(PartialEq, Component)]
enum SettingsButtonAction {
    ChangeRows(bool),
    ChangeColumns(bool),
    ChangeBombCount(bool),
    Preset(Preset),
    SafeStartToggle,
    StartGame,
    Back,
}

#[derive(Component)]
struct SelectedPreset;

pub struct GameOptionsPlugin;

impl Plugin for GameOptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Options), Self::setup_options)
            .add_systems(OnExit(GameState::Options), despawn_all::<OnOptionsScreen>)
            .add_systems(
                Update,
                (
                    Self::preset_button_color,
                    Self::button_actions,
                    Self::display_options,
                )
                    .run_if(in_state(GameState::Options)),
            );
    }
}

impl GameOptionsPlugin {
    fn preset_button_color(
        mut interaction_query: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &SettingsButtonAction,
                Option<&SelectedPreset>,
            ),
            Changed<Interaction>,
        >,
        ui_assets: Res<UiAssets>,
        game_options: Res<GameOptions>,
    ) {
        for (interaction, mut color, action, selected) in interaction_query.iter_mut() {
            let on = match action {
                SettingsButtonAction::SafeStartToggle => game_options.safe_start,
                SettingsButtonAction::Preset(_) => selected.is_some(),
                _ => continue,
            };

            *color = match (interaction, on) {
                (Interaction::Pressed, _) | (_, true) => ui_assets.accent.into(),
                (Interaction::Hovered, false) => ui_assets.accent_alt.into(),
                (Interaction::None, false) => ui_assets.background_alt.into(),
            };
        }
    }

    fn button_actions(
        mut commands: Commands,
        interaction_query: Query<
            (&Interaction, &SettingsButtonAction, Entity),
            Changed<Interaction>,
        >,
        mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedPreset>>,
        mut game_options: ResMut<GameOptions>,
        mut app_state: ResMut<NextState<AppState>>,
        mut game_state: ResMut<NextState<GameState>>,
        ui_assets: Res<UiAssets>,
    ) {
        for (interaction, action, entity) in interaction_query.iter() {
            if *interaction != Interaction::Pressed {
                return;
            }
            match action {
                SettingsButtonAction::StartGame => {
                    commands.remove_resource::<Board>();
                    game_state.set(GameState::Playing);
                }
                SettingsButtonAction::Back => {
                    app_state.set(AppState::Menu);
                    game_state.set(GameState::Inactive);
                }
                SettingsButtonAction::ChangeRows(increase) => {
                    if let Ok((selected_entity, mut selected_color)) =
                        selected_query.get_single_mut()
                    {
                        commands.entity(selected_entity).remove::<SelectedPreset>();
                        *selected_color = ui_assets.background_alt.into();
                    }

                    if *increase {
                        game_options.size.y = game_options.size.y.saturating_add(1);
                    } else {
                        game_options.size.y = game_options.size.y.saturating_sub(1);
                    }
                }
                SettingsButtonAction::ChangeColumns(increase) => {
                    if let Ok((selected_entity, mut selected_color)) =
                        selected_query.get_single_mut()
                    {
                        commands.entity(selected_entity).remove::<SelectedPreset>();
                        *selected_color = ui_assets.background_alt.into();
                    }

                    if *increase {
                        game_options.size.x = game_options.size.x.saturating_add(1);
                    } else {
                        game_options.size.x = game_options.size.x.saturating_sub(1);
                    }
                }
                SettingsButtonAction::ChangeBombCount(increase) => {
                    if let Ok((selected_entity, mut selected_color)) =
                        selected_query.get_single_mut()
                    {
                        commands.entity(selected_entity).remove::<SelectedPreset>();
                        *selected_color = ui_assets.background_alt.into();
                    }

                    if *increase {
                        game_options.bomb_count = game_options.bomb_count.saturating_add(1);
                    } else {
                        game_options.bomb_count = game_options.bomb_count.saturating_sub(1);
                    }
                }
                SettingsButtonAction::Preset(preset) => {
                    if let Ok((selected_entity, mut selected_color)) =
                        selected_query.get_single_mut()
                    {
                        if entity == selected_entity {
                            return;
                        }

                        commands.entity(selected_entity).remove::<SelectedPreset>();
                        *selected_color = ui_assets.background_alt.into();
                    }

                    commands.entity(entity).insert(SelectedPreset);

                    game_options.size = preset.size();
                    game_options.bomb_count = preset.bomb_count();

                    return;
                }
                SettingsButtonAction::SafeStartToggle => {
                    game_options.safe_start = !game_options.safe_start;
                }
            }
        }
    }

    fn display_options(
        mut commands: Commands,
        mut fields_query: Query<(&mut Text, &SettingsTextField)>,
        game_options: Res<GameOptions>,
        mut buttons_query: Query<(&SettingsButtonAction, &mut BackgroundColor, Entity)>,
        ui_assets: Res<UiAssets>,
    ) {
        for (mut text, field) in fields_query.iter_mut() {
            text.sections[0].value = match field {
                SettingsTextField::Rows => game_options.size.y.to_string(),
                SettingsTextField::Columns => game_options.size.x.to_string(),
                SettingsTextField::BombCount => game_options.bomb_count.to_string(),
            }
        }

        for preset in Preset::values() {
            if preset.size() != game_options.size || preset.bomb_count() != game_options.bomb_count
            {
                continue;
            }
            for (button_action, mut color, entity) in buttons_query.iter_mut() {
                if *button_action != SettingsButtonAction::Preset(preset) {
                    continue;
                }

                commands.entity(entity).insert(SelectedPreset);
                *color = ui_assets.accent.into();
                return;
            }
        }
    }

    fn setup_options(
        mut commands: Commands,
        game_options: Option<Res<GameOptions>>,
        ui_assets: Res<UiAssets>,
    ) {
        let game_options = match game_options {
            Some(o) => o.clone(),
            None => {
                commands.insert_resource(GameOptions::default());
                GameOptions::default()
            }
        };

        let flex_column = NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        };

        let flex_row = NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        };

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

        let body = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..Default::default()
                    },
                    background_color: ui_assets.background.into(),
                    ..Default::default()
                },
                OnOptionsScreen,
            ))
            .id();

        let settings_column = commands.spawn(flex_column.clone()).id();
        let title = commands
            .spawn(TextBundle::from_section(
                "Game options",
                ui_assets.style_title(),
            ))
            .id();
        let heading_presets = commands
            .spawn(TextBundle::from_section("Presets:", ui_assets.style_h1()))
            .id();
        let presets_row = commands.spawn(flex_row.clone()).id();
        let rows_row = commands.spawn(flex_row.clone()).id();
        let columns_row = commands.spawn(flex_row.clone()).id();
        let bomb_count_row = commands.spawn(flex_row.clone()).id();
        let safe_start_row = commands.spawn(flex_row.clone()).id();
        let start_game_button = commands
            .spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: ui_assets.accent_alt.into(),
                    ..Default::default()
                },
                SettingsButtonAction::StartGame,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Start Game",
                    button_text_style.clone(),
                ));
            })
            .id();
        let back_button = commands
            .spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: ui_assets.background_alt.into(),
                    ..Default::default()
                },
                SettingsButtonAction::Back,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section("Back", button_text_style.clone()));
            })
            .id();

        commands.entity(body).push_children(&[settings_column]);

        commands.entity(settings_column).push_children(&[
            title,
            heading_presets,
            presets_row,
            rows_row,
            columns_row,
            bomb_count_row,
            safe_start_row,
            start_game_button,
            back_button,
        ]);

        for preset in Preset::values() {
            let selected = game_options.size == preset.size()
                && game_options.bomb_count == preset.bomb_count();

            let background_color = if selected {
                ui_assets.accent.into()
            } else {
                ui_assets.background_alt.into()
            };

            let button = commands
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color,
                        ..Default::default()
                    },
                    SettingsButtonAction::Preset(preset),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        preset.to_string(),
                        TextStyle {
                            color: ui_assets.foreground,
                            ..button_text_style.clone()
                        },
                    ));
                })
                .id();

            if selected {
                commands.entity(button).insert(SelectedPreset);
            }

            commands.entity(presets_row).push_children(&[button]);
        }

        let arrows_column = NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                height: Val::Percent(100.),
                ..Default::default()
            },
            ..Default::default()
        };

        let arrow_button = ButtonBundle {
            style: Style {
                width: Val::Px(20.),
                height: Val::Px(20.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: ui_assets.background.into(),
            ..Default::default()
        };

        let mut load_setting =
            |row_entity: Entity,
             text: &str,
             value: u32,
             field: SettingsTextField,
             increase_button: SettingsButtonAction,
             decrease_button: SettingsButtonAction| {
                let text_entity = commands
                    .spawn(
                        TextBundle::from_section(text, ui_assets.style_h1()).with_style(Style {
                            width: Val::Px(250.),
                            ..Default::default()
                        }),
                    )
                    .id();
                let field_entity = commands
                    .spawn((
                        TextBundle::from_section(value.to_string(), ui_assets.style_h1_accent())
                            .with_style(Style {
                                width: Val::Px(60.),
                                ..Default::default()
                            }),
                        field,
                    ))
                    .id();
                let buttons_column_entity = commands.spawn(arrows_column.clone()).id();
                let arrow_up_button = commands
                    .spawn((arrow_button.clone(), increase_button))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "▲",
                            ui_assets.style_text_accent_alt(),
                        ));
                    })
                    .id();
                let arrow_down_button = commands
                    .spawn((arrow_button.clone(), decrease_button))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "▼",
                            ui_assets.style_text_accent_alt(),
                        ));
                    })
                    .id();

                commands.entity(row_entity).push_children(&[
                    text_entity,
                    field_entity,
                    buttons_column_entity,
                ]);

                commands
                    .entity(buttons_column_entity)
                    .push_children(&[arrow_up_button, arrow_down_button]);
            };

        load_setting(
            rows_row,
            "Rows:",
            game_options.size.y,
            SettingsTextField::Rows,
            SettingsButtonAction::ChangeRows(true),
            SettingsButtonAction::ChangeRows(false),
        );

        load_setting(
            columns_row,
            "Columns:",
            game_options.size.x,
            SettingsTextField::Columns,
            SettingsButtonAction::ChangeColumns(true),
            SettingsButtonAction::ChangeColumns(false),
        );

        load_setting(
            bomb_count_row,
            "Bomb count:",
            game_options.bomb_count,
            SettingsTextField::BombCount,
            SettingsButtonAction::ChangeBombCount(true),
            SettingsButtonAction::ChangeBombCount(false),
        );

        let safe_start_heading = commands
            .spawn(
                TextBundle::from_section("Safe start:", ui_assets.style_h1()).with_style(Style {
                    margin: UiRect::right(Val::Px(20.)),
                    ..Default::default()
                }),
            )
            .id();

        let safe_start_button = commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(50.),
                        height: Val::Px(50.),
                        border: UiRect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    border_color: ui_assets.background_alt.into(),
                    background_color: if game_options.safe_start {
                        ui_assets.accent.into()
                    } else {
                        ui_assets.background_alt.into()
                    },
                    ..Default::default()
                },
                SettingsButtonAction::SafeStartToggle,
            ))
            .id();

        commands
            .entity(safe_start_row)
            .push_children(&[safe_start_heading, safe_start_button]);
    }
}
