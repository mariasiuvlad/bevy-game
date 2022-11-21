mod character_controller;
mod input_map;
mod monkey;
mod test_level;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use character_controller::CharacterControllerPlugin;
use monkey::MonkeyPlugin;
use test_level::setup_test_level;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.2, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default()) // causes crash with bevy 0.9
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_test_level)
        .add_plugin(MonkeyPlugin)
        .add_plugin(CharacterControllerPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn(Camera3dBundle::default());
}
