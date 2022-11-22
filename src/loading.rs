use crate::AppState;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AssetsLoading(Vec<HandleUntyped>);

pub fn load_assets(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    // we can have different asset types
    let character_model: Handle<Scene> = server.load("monkey_warrior.glb#Scene0");

    // add them all to our collection for tracking
    loading.0.push(character_model.clone_untyped());
}

pub fn check_assets_ready(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut app_state: ResMut<State<AppState>>,
) {
    use bevy::asset::LoadState;

    match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Failed => {
            // one of our assets had an error
        }
        LoadState::Loaded => {
            // all assets are now ready

            // this might be a good place to transition into your in-game state
            app_state.set(AppState::InGame).unwrap();

            // remove the resource to drop the tracking handles
            commands.remove_resource::<AssetsLoading>();
            // (note: if you don't have any other handles to the assets
            // elsewhere, they will get unloaded after this)
        }
        _ => {
            info!("Loading assets...");
            // NotLoaded/Loading: not fully ready yet
        }
    }
}
