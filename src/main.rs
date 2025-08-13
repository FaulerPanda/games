pub mod enums;
pub mod functions;
pub mod game;
pub mod loading_screen;
pub mod menus;
pub mod structs;

use crate::enums::GameState;

use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};

use game::PongPlugin;
use loading_screen::LoadingScreenPlugin;
use menus::{game_menu::GameMenuPlugin, main_menu::MainMenuPlugin};

fn main() {
    App::new()
        // setup Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Games".to_string(),
                // mode: WindowMode::Fullscreen(
                //     MonitorSelection::Primary,
                //     VideoModeSelection::Current,
                // ),
                ..default()
            }),

            ..default()
        }))
        .add_plugins(FramepacePlugin)
        .insert_resource(FramepaceSettings {
            limiter: Limiter::from_framerate(60.0),
        })
        //rest for plugins
        .add_plugins(MainMenuPlugin)
        .add_plugins(GameMenuPlugin)
        .add_plugins(PongPlugin)
        .add_plugins(LoadingScreenPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .run();
}

/// spawns the game Camera
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
