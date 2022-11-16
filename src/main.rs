mod monkey;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use monkey::monkey::MonkeyPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.2, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_plugin(MonkeyPlugin)
        .run();
}

fn setup(mut commands: Commands) {
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
    commands
        .spawn(())
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -1.0, 0.0)));
}
