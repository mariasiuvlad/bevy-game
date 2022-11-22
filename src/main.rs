mod character_controller;
mod graphics;
mod input_map;
mod maps;
mod monkey;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use character_controller::CharacterControllerPlugin;
use monkey::MonkeyPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(graphics::setup_camera)
        .add_startup_system(maps::hello_world::setup_level)
        .add_plugin(MonkeyPlugin)
        .add_plugin(CharacterControllerPlugin)
        .run();
}
