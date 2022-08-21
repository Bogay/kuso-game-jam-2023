use crate::audio::plugin::MyAudioPlugin;
use crate::config::config_log::LogConfig;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::DefaultPlugins;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use egui::*;
use heron::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::game::GamePlugin;
use crate::game_ended::GameEndedPlugin;
use crate::input::InputsPlugin;
use crate::loading::state::LoadingPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::states::{handle_escape, log_state_changes, AppState};
use crate::window_event_handler::handle_window;

mod audio;
mod cleanup;
mod config;
mod game;
mod game_ended;
mod grid;
mod input;
mod loading;
mod main_menu;
mod states;
mod window_event_handler;

pub const GAME_NAME: &str = "Bevy Jam 2 Game";

fn main() {
    App::new()
        .insert_resource(bevy::log::LogSettings {
            level: LogConfig::load_from_file().level.parse().unwrap(),
            ..Default::default()
        })
        .insert_resource(WindowDescriptor {
            title: GAME_NAME.to_string(),
            mode: WindowMode::Windowed,
            ..default()
        })
        .add_loopless_state(AppState::Loading)
        .add_state(game::GameResult::Won)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MyAudioPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameEndedPlugin)
        .add_plugin(InputsPlugin)
        .add_system(handle_window)
        .add_system(log_state_changes)
        .add_system(handle_escape)
        .run();
}

// Define your physics layers
#[derive(PhysicsLayer)]
enum PhysLayer {
    World,
    Draggables,
}
