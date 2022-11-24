use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::loading::AssetsLoading;

pub fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    loading: Res<AssetsLoading>,
) {
    let stairs_mesh: Handle<Mesh> = loading.0[1].clone().typed::<Mesh>();
    let level_mesh: Handle<Mesh> = loading.0[2].clone().typed::<Mesh>();
    let stairs_collider = Collider::from_bevy_mesh(
        &meshes.get(&stairs_mesh).unwrap(),
        &ComputedColliderShape::TriMesh,
    )
    .unwrap();
    let level_collider = Collider::from_bevy_mesh(
        &meshes.get(&level_mesh).unwrap(),
        &ComputedColliderShape::TriMesh,
    )
    .unwrap();

    // light
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(20.0, 10.0, 20.0),
            ..default()
        },
        Name::from("Light #1"),
    ));
    commands.spawn((
        Name::from("Light #2"),
        PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(-20.0, 10.0, 20.0),
            ..default()
        },
    ));

    // commands.spawn((
    //     Name::from("Hello World"),
    //     level_collider,
    //     TransformBundle::from(
    //         Transform::from_xyz(0.0, -5.0, 0.0).with_scale(Vec3::new(10., 0.05, 10.)),
    //     ),
    // ));

    /* Create companion cube */
    commands.spawn((
        RigidBody::Dynamic,
        Name::from("Companion Cube"),
        Collider::cuboid(0.2, 0.2, 0.2),
        TransformBundle::from_transform(Transform::from_xyz(-5.0, 0.0, 0.0)),
    ));

    /* Create stairs */
    commands.spawn((
        Name::from("Stairs"),
        stairs_collider,
        TransformBundle::from(Transform::from_xyz(5.0, 0.0, 0.0)),
    ));

    /* Create the ground. */
    commands.spawn((
        Name::from("Ground"),
        Collider::cuboid(25.0, 0.2, 25.0),
        RigidBody::Fixed,
        TransformBundle::from_transform(Transform::from_xyz(0.0, -1.0, 0.0)),
    ));

    /* Create pillars */
    commands.spawn((
        Name::from("Pillar #1"),
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
            transform: Transform::from_xyz(20., 1., 20.),
            ..default()
        },
    ));
    commands.spawn((
        Name::from("Pillar #2"),
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
            transform: Transform::from_xyz(-20., 1.25, 20.),
            ..default()
        },
    ));
    commands.spawn((
        Name::from("Pillar #3"),
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
            transform: Transform::from_xyz(20., 1.25, -20.),
            ..default()
        },
    ));
    commands.spawn((
        Name::from("Pillar #4"),
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
            transform: Transform::from_xyz(-20., 1.25, -20.),
            ..default()
        },
    ));
}
