use crate::{input_map::InputMap, monkey::Animations, AppState};
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

const MAX_SPEED: f32 = 5.;
const _ACCELERATION: f32 = 200.;
const _MAX_ACCEL_FORCE: f32 = 150.;

#[derive(Component, Debug, Default)]
pub struct InputState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub run: bool,
    pub jump: bool,
    pub up: bool,
    pub down: bool,
}

#[derive(Component, Debug)]
pub struct CharacterInput {
    pub input_map: InputMap,
    pub fly: bool,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub jump_speed: f32,
    pub input_state: InputState,
}

impl Default for CharacterInput {
    fn default() -> Self {
        Self {
            input_map: InputMap::default(),
            input_state: InputState::default(),
            fly: false,
            walk_speed: 5.0,
            run_speed: 9.0,
            jump_speed: 6.0,
        }
    }
}

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_cursor_grab))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(handle_input)
                    .with_system(input_to_movement)
                    .with_system(floating)
                    .with_system(input_to_turning)
                    .with_system(cursor_grab_system)
                    .with_system(move_camera)
                    .with_system(mouse_button_input),
            );
    }
}

pub fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut input_query: Query<(&mut InputState, &InputMap)>,
) {
    for (mut input_state, input_map) in input_query.iter_mut() {
        if keyboard_input.pressed(input_map.key_forward) {
            input_state.forward = true;
        }
        if keyboard_input.pressed(input_map.key_backward) {
            input_state.backward = true;
        }
        if keyboard_input.pressed(input_map.key_right) {
            input_state.right = true;
        }
        if keyboard_input.pressed(input_map.key_left) {
            input_state.left = true;
        }
        if keyboard_input.pressed(input_map.key_run) {
            input_state.run = true;
        }
        if keyboard_input.just_pressed(input_map.key_jump) {
            input_state.jump = true;
        }
        if keyboard_input.pressed(input_map.key_fly_up) {
            input_state.up = true;
        }
        if keyboard_input.pressed(input_map.key_fly_down) {
            input_state.down = true;
        }
    }
}

pub fn input_to_movement(
    mut character: Query<(&Transform, &Velocity, &mut ExternalForce, &mut InputState)>,
) {
    for (transform, velocity, mut force, mut input_state) in character.iter_mut() {
        let mut final_linvel = Vec3::ZERO;

        if input_state.forward {
            final_linvel += -transform.forward();
        }
        if input_state.backward {
            final_linvel += transform.forward();
        }
        if input_state.right {
            final_linvel += -transform.local_x();
        }
        if input_state.left {
            final_linvel += transform.local_x();
        }

        final_linvel = final_linvel.normalize_or_zero();

        let mut goal_linvel = final_linvel * MAX_SPEED;
        goal_linvel.y = velocity.linvel.y;

        let needed = goal_linvel - velocity.linvel;

        if input_state.jump {
            force.force.y += 20.0;
        }

        force.force.x = needed.x;
        force.force.z = needed.z;

        *input_state = InputState::default();
    }
}

fn floating(
    rapier_context: Res<RapierContext>,
    mut character: Query<(
        Entity,
        &Transform,
        &Velocity,
        &mut ExternalForce,
        &InputState,
    )>,
) {
    for (entity, transform, velocity, mut force, input_state) in character.iter_mut() {
        let ray_dir = Vec3::NEG_Y;
        let ride_height: f32 = 0.75;
        let max_toi = 1.5;
        let solid = true;

        let _spring_strength = 10.;
        let _spring_damper = 1.;

        let filter = QueryFilter {
            exclude_collider: Some(entity),
            ..default()
        };
        let ray_origin = transform.translation;
        if !input_state.jump {
            if let Some((_, toi)) =
                rapier_context.cast_ray(ray_origin, ray_dir, max_toi, solid, filter)
            {
                let distance = ray_dir * toi;
                let delta = ride_height - distance.y.abs();
                let spring_force =
                    (delta * _spring_strength) - (velocity.linvel.y * _spring_damper);

                force.force.y = spring_force;
            } else {
                force.force.y = 0.;
            }
        }
    }
}

fn input_to_turning(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut controller_query: Query<&mut Velocity, With<InputState>>,
) {
    for mut velocity in controller_query.iter_mut() {
        let mut delta = Vec2::ZERO;
        for motion in mouse_motion_events.iter() {
            // NOTE: -= to invert
            delta -= motion.delta;
        }

        let angvel = Vec3::new(0., delta.x.clamp(-10., 10.), 0.);
        velocity.angvel = angvel;
    }
}

fn setup_cursor_grab(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_grab_mode(bevy::window::CursorGrabMode::Locked);
    window.set_cursor_visibility(false);
}

fn cursor_grab_system(mut windows: ResMut<Windows>, key: Res<Input<KeyCode>>) {
    let window = windows.get_primary_mut().unwrap();

    if key.just_pressed(KeyCode::T) {
        window.set_cursor_grab_mode(bevy::window::CursorGrabMode::Locked);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_grab_mode(bevy::window::CursorGrabMode::None);
        window.set_cursor_visibility(true);
    }
}

fn move_camera(
    character_query: Query<&Transform, With<CharacterInput>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<CharacterInput>)>,
) {
    for character_transform in character_query.iter() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation = character_transform.translation
                + character_transform.forward() * 5.0
                + character_transform.local_y();
            camera_transform.look_at(
                character_transform.translation - character_transform.forward() * 5.0,
                Vec3::Y,
            );
        }
    }
}

fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    animations: Res<Animations>,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    for mut animation_player in animation_players.iter_mut() {
        if buttons.just_pressed(MouseButton::Left) {
            animation_player.start(animations.0[0].clone_weak());
        }
    }
}

/* Cast a ray inside of a system. */
fn cast_downward_ray(
    rapier_context: Res<RapierContext>,
    mut characters: Query<(Entity, &Name, &Transform, &mut ExternalForce), With<InputState>>,
) {
    let ray_dir = Vec3::NEG_Y;
    let ride_height: f32 = 1.;
    let max_toi = 4.0;
    let solid = true;

    for (entity, name, transform, mut force) in characters.iter_mut() {
        let filter = QueryFilter {
            exclude_collider: Some(entity),
            ..default()
        };
        let ray_origin = transform.translation;
        if let Some((entity, toi)) =
            rapier_context.cast_ray(ray_origin, ray_dir, max_toi, solid, filter)
        {
            // The first collider hit has the entity `entity` and it hit after
            // the ray travelled a distance equal to `ray_dir * toi`.
            let hit_point = ray_origin + ray_dir * toi;
            let distance = ray_dir * toi;
            let spring_force = Vec3::Y * (ride_height - distance.y.abs()) * 0.5;
            println!(
                "Entity {:?} hit at point {} with a distance of {} generating a spring force of {} - {}",
                entity, hit_point, distance, spring_force, name
            );
            force.force = spring_force;
        }
    }
}
