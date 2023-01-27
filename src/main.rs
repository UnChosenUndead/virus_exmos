use bevy::prelude::KeyCode::H;
use bevy::prelude::*;
use bevy::window::{CompositeAlphaMode, CursorGrabMode, PresentMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::scenes::basic_scene::BasicScenePlugin;

mod entity;
mod plugins;
mod scenes;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        .add_plugin(BasicScenePlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Virus Exmos".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin)
        .run();
}
