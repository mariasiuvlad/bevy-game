pub mod monkey {
    use bevy::prelude::*;
    use bevy_inspector_egui::{Inspectable, RegisterInspectable};
    use bevy_rapier3d::prelude::*;
    use std::ops::Add;

    #[derive(Component)]
    struct Speed(f32);

    #[derive(Inspectable, Debug)]
    enum AnimationStates {
        Idle,
        Attacking,
    }

    #[derive(Component)]
    struct AttackTime(Timer);

    #[derive(Inspectable, Component)]
    struct AnimationState(AnimationStates);

    #[derive(Component)]
    struct Player;

    #[derive(Resource)]
    struct Animations(Vec<Handle<AnimationClip>>);

    pub struct MonkeyPlugin;

    impl Plugin for MonkeyPlugin {
        fn build(&self, app: &mut App) {
            app.add_startup_system(spawn_monkey)
                .register_inspectable::<AnimationState>()
                .add_system(keyboard_input);
        }
    }

    fn spawn_monkey(mut commands: Commands, assets: Res<AssetServer>) {
        let my_gltf = assets.load("monkey_warrior.glb#Scene0");
        commands.insert_resource(Animations(vec![
            assets.load("monkey_warrior.glb#Animation0")
        ]));
        commands.spawn((
            Player,
            Speed(2.0),
            AnimationState(AnimationStates::Idle),
            RigidBody::Dynamic,
            Collider::ball(0.5),
            Restitution::coefficient(0.7),
            SceneBundle {
                scene: my_gltf,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
        ));
    }

    fn keyboard_input(
        mut player_position: Query<(&Speed, &mut Transform, &mut AnimationState), With<Player>>,
        mut animation_players: Query<&mut AnimationPlayer>,
        animations: Res<Animations>,
        time: Res<Time>,
        keys: Res<Input<KeyCode>>,
    ) {
        let query_result = player_position.get_single_mut();
        match query_result {
            Ok((speed, mut transform, mut animation_state)) => {
                if keys.any_just_released(vec![KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
                    animation_state.0 = AnimationStates::Idle;
                }
                if keys.just_pressed(KeyCode::Space) {
                    for mut animation_player in animation_players.iter_mut() {
                        animation_player.start(animations.0[0].clone());
                    }
                    animation_state.0 = AnimationStates::Attacking;
                }
                // movement
                if keys.pressed(KeyCode::W) {
                    // W is being held down
                    let offset = transform.local_z() * speed.0 * time.delta_seconds();
                    transform.translation = transform.translation.add(offset);
                }
                if keys.pressed(KeyCode::S) {
                    let offset = transform.local_z() * -speed.0 * time.delta_seconds();
                    transform.translation = transform.translation.add(offset);
                }
                if keys.pressed(KeyCode::A) {
                    let offset = transform.local_x() * speed.0 * time.delta_seconds();
                    transform.translation = transform.translation.add(offset);
                }
                if keys.pressed(KeyCode::D) {
                    let offset = transform.local_x() * -speed.0 * time.delta_seconds();
                    transform.translation = transform.translation.add(offset);
                }
                // turning
                if keys.pressed(KeyCode::Q) {
                    let offset = 4.0 * time.delta_seconds();
                    transform.rotate_y(offset);
                }
                if keys.pressed(KeyCode::E) {
                    let offset = 4.0 * -time.delta_seconds();
                    transform.rotate_y(offset);
                }
            }
            Err(error) => {
                warn!("Failed to find player reference, {}", error.to_string())
            }
        }
    }
}
