use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn((Name::from("Player Camera"), Camera3dBundle::default()));
}
