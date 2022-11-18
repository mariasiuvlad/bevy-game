mod character_controller;
mod input_map;
mod monkey;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use character_controller::CharacterControllerPlugin;
use monkey::MonkeyPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.2, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default()) // causes crash with bevy 0.9
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_plugin(MonkeyPlugin)
        .add_plugin(CharacterControllerPlugin)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-5.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    });

    /* Create the ground. */
    let ground_mesh = meshes.add(shape::Plane { size: 50. }.into());
    commands.spawn((
        Collider::cuboid(100.0, 0.1, 100.0),
        RigidBody::Fixed,
        PbrBundle {
            mesh: ground_mesh,
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
    ));

    /* Create pillars */
    commands.spawn((
        Collider::cuboid(5., 5., 5.),
        PbrBundle {
            mesh: meshes.add(shape::Cube { size: 5. }.into()),
            transform: Transform::from_xyz(20., 0., 20.),
            ..default()
        },
    ));
    commands.spawn((
        Collider::cuboid(5., 5., 5.),
        PbrBundle {
            mesh: meshes.add(shape::Cube { size: 5. }.into()),
            transform: Transform::from_xyz(-20., 0., 20.),
            ..default()
        },
    ));
    commands.spawn((
        Collider::cuboid(5., 5., 5.),
        PbrBundle {
            mesh: meshes.add(shape::Cube { size: 5. }.into()),
            transform: Transform::from_xyz(20., 0., -20.),
            ..default()
        },
    ));
    commands.spawn((
        Collider::cuboid(5., 5., 5.),
        PbrBundle {
            mesh: meshes.add(shape::Cube { size: 5. }.into()),
            transform: Transform::from_xyz(-20., 0., -20.),
            ..default()
        },
    ));
}
