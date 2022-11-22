use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{character_controller::CharacterInput, AppState};

#[derive(Component)]
struct Player;

#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);

pub struct MonkeyPlugin;

impl Plugin for MonkeyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_monkey));
    }
}

fn spawn_monkey(mut commands: Commands, assets: Res<AssetServer>) {
    let monkey_model = assets.load("monkey_warrior.glb#Scene0");
    commands.insert_resource(Animations(vec![
        assets.load("monkey_warrior.glb#Animation0")
    ]));
    commands.spawn((
        Name::from("Monkey Warrior"),
        Player,
        RigidBody::Dynamic,
        CharacterInput::default(),
        Velocity::default(),
        KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Absolute(0.5)),
            ..default()
        },
        KinematicCharacterControllerOutput::default(),
        LockedAxes::ROTATION_LOCKED,
        Collider::cuboid(0.5, 1.5, 0.5),
        SceneBundle {
            scene: monkey_model,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::ONE * 0.25),
            ..Default::default()
        },
    ));
}
