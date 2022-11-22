use crate::{input_map::InputMap, monkey::Animations, AppState};
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Debug, Default)]
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
                    .with_system(input_to_controller_based_movement)
                    .with_system(input_to_turning)
                    .with_system(cursor_grab_system)
                    .with_system(move_camera)
                    .with_system(mouse_button_input),
            );
    }
}

pub fn handle_input(
    _time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut controller_query: Query<&mut CharacterInput>,
) {
    for mut controller in controller_query.iter_mut() {
        if keyboard_input.just_pressed(controller.input_map.key_fly) {
            controller.fly = !controller.fly;
        }
        if keyboard_input.pressed(controller.input_map.key_forward) {
            controller.input_state.forward = true;
        }
        if keyboard_input.pressed(controller.input_map.key_backward) {
            controller.input_state.backward = true;
        }
        if keyboard_input.pressed(controller.input_map.key_right) {
            controller.input_state.right = true;
        }
        if keyboard_input.pressed(controller.input_map.key_left) {
            controller.input_state.left = true;
        }
        if keyboard_input.pressed(controller.input_map.key_run) {
            controller.input_state.run = true;
        }
        if keyboard_input.just_pressed(controller.input_map.key_jump) {
            controller.input_state.jump = true;
        }
        if keyboard_input.pressed(controller.input_map.key_fly_up) {
            controller.input_state.up = true;
        }
        if keyboard_input.pressed(controller.input_map.key_fly_down) {
            controller.input_state.down = true;
        }
    }
}

pub fn input_to_controller_based_movement(
    time: Res<Time>,
    mut controller_query: Query<(
        &Transform,
        &mut CharacterInput,
        &mut KinematicCharacterController,
        &mut Velocity,
        &KinematicCharacterControllerOutput,
    )>,
) {
    for (transform, mut input, mut kinematic_controller, mut velocity, output) in
        controller_query.iter_mut()
    {
        let mut translation = Vec3::ZERO;
        let is_grounded = output.grounded;

        if input.input_state.jump && is_grounded {
            let mut jump_force = transform.local_y() * input.jump_speed;
            if input.input_state.forward {
                jump_force += -transform.forward() * input.walk_speed;
            }
            velocity.linvel += jump_force;
        }

        if input.input_state.forward && is_grounded {
            translation += -transform.forward();
        }
        if input.input_state.backward && is_grounded {
            translation += transform.forward();
        }
        if input.input_state.right && is_grounded {
            translation += -transform.local_x();
        }
        if input.input_state.left && is_grounded {
            translation += transform.local_x();
        }

        kinematic_controller.translation = if is_grounded {
            Some(translation * input.walk_speed * time.delta_seconds())
        } else {
            Some(Vec3::ZERO)
        };

        input.input_state = InputState::default();
    }
}

fn input_to_turning(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut controller_query: Query<(&mut Velocity, &CharacterInput)>,
) {
    for (mut velocity, mut _controller) in controller_query.iter_mut() {
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
                + character_transform.local_y() * 2.5;
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
