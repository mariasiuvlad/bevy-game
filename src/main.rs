mod character_controller;
mod graphics;
mod input_map;
mod loading;
mod maps;
mod monkey;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use character_controller::CharacterControllerPlugin;
use loading::{check_assets_ready, load_assets, AssetsLoading};
use monkey::MonkeyPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    InGame,
}

fn main() {
    App::new()
        .init_resource::<AssetsLoading>()
        .add_state(AppState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_assets))
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_assets_ready))
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(graphics::setup_camera)
                .with_system(maps::hello_world::setup_level),
        )
        .add_plugin(MonkeyPlugin)
        .add_plugin(CharacterControllerPlugin)
        .run();
}
