use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::{style::ui_assets::UiAssets, util::despawn_all, AppState};

#[derive(Component)]
struct OnSplashScreen;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), Self::setup)
            .add_systems(Update, Self::advance.run_if(in_state(AppState::Splash)))
            .add_systems(OnExit(AppState::Splash), despawn_all::<OnSplashScreen>);
    }
}

impl SplashPlugin {
    fn setup(mut commands: Commands, ui_assets: Res<UiAssets>) {
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
                OnSplashScreen,
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
                            "Minesweeper",
                            TextStyle {
                                font_size: 80.0,
                                color: ui_assets.foreground,
                                ..Default::default()
                            },
                        ));

                        parent.spawn(TextBundle::from_section(
                            "Press any button to continue.",
                            TextStyle {
                                font_size: 20.0,
                                color: ui_assets.foreground_alt,
                                ..Default::default()
                            },
                        ));
                    });
            });
    }

    fn advance(
        mut key_evr: EventReader<KeyboardInput>,
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        if key_evr.read().any(|key| key.state.is_pressed()) {
            app_state.set(AppState::Menu)
        }
    }
}
