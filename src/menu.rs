use bevy::{app::AppExit, prelude::*};

use crate::{style::ui_assets::UiAssets, util::despawn_all, AppState};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum MenuState {
    Main,
    Settings,
    BoardSettings,
    ColorSettings,
    #[default]
    Inactive,
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnSettingsMenuScreen;

#[derive(Component)]
struct OnBoardSettingsMenuScreen;

#[derive(Component)]
enum MenuButtonAction {
    NewGame,
    EnterSettings,
    ExitSettings,
    ExitGame,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(AppState::Menu), Self::setup_menu)
            .add_systems(OnEnter(MenuState::Main), Self::setup_main_menu)
            .add_systems(OnExit(MenuState::Main), despawn_all::<OnMainMenuScreen>)
            .add_systems(OnEnter(MenuState::Settings), Self::setup_settings_menu)
            .add_systems(
                OnExit(MenuState::Settings),
                despawn_all::<OnSettingsMenuScreen>,
            )
            .add_systems(
                OnEnter(MenuState::BoardSettings),
                Self::setup_board_settings_menu,
            )
            .add_systems(
                OnExit(MenuState::BoardSettings),
                despawn_all::<OnBoardSettingsMenuScreen>,
            )
            .add_systems(
                OnEnter(MenuState::ColorSettings),
                Self::setup_color_settings_menu,
            )
            .add_systems(
                OnExit(MenuState::ColorSettings),
                despawn_all::<OnBoardSettingsMenuScreen>,
            )
            .add_systems(
                Update,
                Self::button_actions.run_if(in_state(AppState::Menu)),
            );
    }
}

impl MenuPlugin {
    #[allow(clippy::type_complexity)]
    fn button_actions(
        interactions: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_evw: EventWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        for (interaction, menu_button_action) in interactions.iter() {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::NewGame => {
                        app_state.set(AppState::Game);
                        menu_state.set(MenuState::Inactive);
                    }
                    MenuButtonAction::EnterSettings => {
                        menu_state.set(MenuState::Settings);
                    }
                    MenuButtonAction::ExitSettings => {
                        menu_state.set(MenuState::Main);
                    }
                    MenuButtonAction::ExitGame => {
                        app_exit_evw.send(AppExit);
                    }
                }
            }
        }
    }

    fn setup_menu(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    fn setup_main_menu(mut commands: Commands, ui_assets: Res<UiAssets>) {
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
            ..Default::default()
        };

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..Default::default()
                    },
                    background_color: ui_assets.accent.into(),
                    ..Default::default()
                },
                OnMainMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        // Game title
                        parent.spawn(
                            TextBundle::from_section(
                                "Minesweeper",
                                TextStyle {
                                    color: ui_assets.foreground,
                                    font_size: 80.,
                                    ..Default::default()
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::all(Val::Px(50.)),
                                ..Default::default()
                            }),
                        );

                        // New game button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: ui_assets.background.into(),
                                    ..Default::default()
                                },
                                MenuButtonAction::NewGame,
                            ))
                            .with_children(|parent| {
                                // TODO: Add icon
                                parent.spawn(TextBundle::from_section(
                                    "New Game",
                                    button_text_style.clone(),
                                ));
                            });

                        // Settings button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: ui_assets.background.into(),
                                    ..Default::default()
                                },
                                MenuButtonAction::EnterSettings,
                            ))
                            .with_children(|parent| {
                                // TODO: Add icon
                                parent.spawn(TextBundle::from_section(
                                    "Settings",
                                    button_text_style.clone(),
                                ));
                            });

                        // Quit game button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: ui_assets.background.into(),
                                    ..Default::default()
                                },
                                MenuButtonAction::ExitGame,
                            ))
                            .with_children(|parent| {
                                // TODO: Add icon
                                parent.spawn(TextBundle::from_section(
                                    "Quit",
                                    button_text_style.clone(),
                                ));
                            });
                    });
            });
    }

    fn setup_settings_menu(mut commands: Commands, ui_assets: Res<UiAssets>) {
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
            ..Default::default()
        };

        commands
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
                OnSettingsMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "TODO: Settings not implemented",
                            TextStyle {
                                font_size: 60.,
                                color: ui_assets.accent,
                                ..Default::default()
                            },
                        ));

                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style,
                                    background_color: ui_assets.background_alt.into(),
                                    ..Default::default()
                                },
                                MenuButtonAction::ExitSettings,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section("Back", button_text_style));
                            });
                    });
            });
    }

    fn setup_board_settings_menu() {
        todo!()
    }

    fn setup_color_settings_menu() {
        todo!()
    }
}
