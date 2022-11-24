use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    character_controller::{CharacterInput, InputState},
    input_map::InputMap,
    AppState,
};

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
    let _monkey_model: Handle<Scene> = assets.load("monkey_warrior.glb#Scene0");
    commands.insert_resource(Animations(vec![
        assets.load("monkey_warrior.glb#Animation0")
    ]));
    commands.spawn((
        Name::from("Monkey Warrior"),
        Player,
        RigidBody::Dynamic,
        CharacterInput::default(),
        InputMap::default(),
        InputState::default(),
        GravityScale(1.),
        Velocity {
            linvel: Vec3::ZERO,
            angvel: Vec3::ZERO,
        },
        ExternalForce::default(),
        ExternalImpulse::default(),
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        Collider::capsule(Vec3::ZERO, Vec3::new(0., 0.25, 0.), 0.125),
        TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
        // SceneBundle {
        //     scene: _monkey_model,
        //     // transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::ONE * 0.25),
        //     ..Default::default()
        // },
    ));
}
