use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_test_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(20.0, 10.0, 20.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-20.0, 10.0, 20.0),
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
        Collider::cuboid(2.5, 2.5, 2.5),
        PbrBundle {
            material: materials.add(StandardMaterial {
                base_color: Color::Rgba {
                    alpha: 1.,
                    red: 1.,
                    green: 1.,
                    blue: 1.,
                },
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            mesh: meshes.add(shape::Cube { size: 5. }.into()),
            transform: Transform::from_xyz(20., 0., 20.),
            ..default()
        },
    ));
    commands.spawn((
        Collider::cuboid(2.5, 2.5, 2.5),
        PbrBundle {
            material: materials.add(StandardMaterial {
                base_color: Color::Rgba {
                    alpha: 1.,
                    red: 1.,
                    green: 1.,
                    blue: 1.,
                },
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            mesh: meshes.add(shape::Cube { size: 5. }.into()),
            transform: Transform::from_xyz(-20., 0., 20.),
            ..default()
        },
    ));
    commands.spawn((
        Collider::cuboid(2.5, 2.5, 2.5),
        PbrBundle {
            material: materials.add(StandardMaterial {
                base_color: Color::Rgba {
                    alpha: 1.,
                    red: 1.,
                    green: 1.,
                    blue: 1.,
                },
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            mesh: meshes.add(shape::Cube { size: 5. }.into()),
            transform: Transform::from_xyz(20., 0., -20.),
            ..default()
        },
    ));
    commands.spawn((
        Collider::cuboid(2.5, 2.5, 2.5),
        PbrBundle {
            material: materials.add(StandardMaterial {
                base_color: Color::Rgba {
                    alpha: 1.,
                    red: 1.,
                    green: 1.,
                    blue: 1.,
                },
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            mesh: meshes.add(shape::Cube { size: 5. }.into()),
            transform: Transform::from_xyz(-20., 0., -20.),
            ..default()
        },
    ));
}
