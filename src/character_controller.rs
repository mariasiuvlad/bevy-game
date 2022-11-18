use crate::input_map::InputMap;
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
pub struct CharacterController {
    pub input_map: InputMap,
    pub fly: bool,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub jump_speed: f32,
    pub velocity: Vec3,
    pub jumping: bool,
    pub input_state: InputState,
}

impl Default for CharacterController {
    fn default() -> Self {
        Self {
            input_map: InputMap::default(),
            fly: false,
            walk_speed: 10.0,
            run_speed: 8.0,
            jump_speed: 6.0,
            velocity: Vec3::ZERO,
            jumping: false,
            input_state: InputState::default(),
        }
    }
}

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_input)
            // .add_system(input_to_force_based_movement)
            .add_system(input_to_velocity_based_movement)
            .add_system(cursor_grab_system)
            .add_system(move_camera);
    }
}

pub fn handle_input(
    _time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut controller_query: Query<&mut CharacterController>,
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

pub fn input_to_velocity_based_movement(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut controller_query: Query<(&Transform, &mut CharacterController, &mut Velocity)>,
) {
    for (transform, mut controller, mut velocity) in controller_query.iter_mut() {
        let mut vertical_velocity = velocity.linvel.y;

        if controller.input_state.jump {
            vertical_velocity = controller.jump_speed;
        }

        let mut final_velocity = Vec3::ZERO;
        if controller.input_state.forward {
            final_velocity += -transform.forward() * controller.walk_speed;
        }
        if controller.input_state.backward {
            final_velocity += transform.forward() * controller.walk_speed;
        }
        if controller.input_state.right {
            final_velocity += -transform.local_x() * controller.walk_speed;
        }
        if controller.input_state.left {
            final_velocity += transform.local_x() * controller.walk_speed;
        }

        velocity.linvel = Vec3::new(final_velocity.x, vertical_velocity, final_velocity.z);

        let mut delta = Vec2::ZERO;
        for motion in mouse_motion_events.iter() {
            // NOTE: -= to invert
            delta -= motion.delta;
        }

        let angvel = Vec3::new(0., delta.x / 2., 0.);
        velocity.angvel = angvel;

        controller.input_state = InputState::default();
    }
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
    character_query: Query<&Transform, With<CharacterController>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<CharacterController>)>,
) {
    for character_transform in character_query.iter() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation = character_transform.translation
                + character_transform.forward() * 20.0
                + character_transform.local_y() * 10.0;
            camera_transform.look_at(character_transform.translation, Vec3::Y);
        }
    }
}
