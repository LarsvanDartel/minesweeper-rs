mod game;
mod menu;
mod splash;
mod style;
mod util;

use bevy::{prelude::*, window::WindowTheme};
use style::{colors::NordDark, game_assets::GameAssets, ui_assets::UiAssets};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Splash,
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Minesweeper".to_string(),
                resolution: (850., 850.).into(),
                window_theme: Some(WindowTheme::Dark),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .init_state::<AppState>()
        .add_systems(Startup, (setup_camera, load_assets))
        .add_plugins((splash::SplashPlugin, menu::MenuPlugin, game::GamePlugin))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn load_assets(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.insert_resource(
        UiAssets::from_colorscheme::<NordDark>()
            .with_font(asset_server.load("fonts/FiraCodeNerdFont-SemiBold.ttf")),
    );
    commands.insert_resource(
        GameAssets::from_colorscheme::<NordDark>()
            .with_font(asset_server.load("fonts/BigBlueTermPlusNerdFont-Regular.ttf")),
    );
}
