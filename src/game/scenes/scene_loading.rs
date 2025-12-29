use bevy::prelude::*;

use crate::game::{assets::{AssetResourceLoading, ResourceHandles, asset_resources::{PlayerAssets, SpaceAssets}}, scenes::GameScene};

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect)]
#[source(GameScene = GameScene::Loading)]
pub enum LoadingState {
    #[default]
    StillLoading,
    DoneLoading,
}

pub fn plugin_scene_loading(app: &mut App) {
    app.add_sub_state::<LoadingState>()
        .add_systems(OnEnter(GameScene::Loading), spawn_loading_screen)
        .add_systems(OnExit(GameScene::Loading), teardown_loading_screen)
        .add_systems(
            Update,
            transition_from_loading.run_if(in_state(GameScene::Loading).and(assets_done_loading)),
        );
    app.add_systems(Update, check_loading_status);
    app.register_type::<PlayerAssets>();
    app.load_asset_resource::<PlayerAssets>();
    app.load_asset_resource::<SpaceAssets>();
}

fn check_loading_status(handles: Res<ResourceHandles>){
    if !handles.is_all_loaded(){
        info!("Resources are still loading...");
    }
}

fn assets_done_loading(resource_handles: Res<ResourceHandles>) -> bool{
    resource_handles.is_all_loaded()
}

pub fn spawn_loading_screen() {}
pub fn transition_from_loading(mut next_state: ResMut<NextState<GameScene>>) {
    next_state.set(GameScene::InGame);
}
pub fn teardown_loading_screen() {}
