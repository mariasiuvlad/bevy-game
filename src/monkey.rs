use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::character_controller::CharacterController;

#[derive(Component)]
struct Player;

#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);

pub struct MonkeyPlugin;

impl Plugin for MonkeyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_monkey);
    }
}

fn spawn_monkey(mut commands: Commands, assets: Res<AssetServer>) {
    let monkey_model = assets.load("monkey_warrior.glb#Scene0");
    commands.insert_resource(Animations(vec![
        assets.load("monkey_warrior.glb#Animation0")
    ]));
    commands.spawn((
        Player,
        RigidBody::Dynamic,
        Velocity {
            linvel: Vec3::ZERO,
            angvel: Vec3::ZERO,
        },
        GravityScale(3.0),
        Collider::cuboid(1.5, 1.0, 1.5),
        LockedAxes::ROTATION_LOCKED,
        CharacterController::default(),
        SceneBundle {
            scene: monkey_model,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ));
}
