use bevy::prelude::*;

use crate::game::scenes::GameScene;

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
            transition_from_loading.run_if(in_state(GameScene::Loading)),
        );
}

pub fn spawn_loading_screen() {}
pub fn transition_from_loading(mut next_state: ResMut<NextState<GameScene>>) {
    next_state.set(GameScene::InGame);
}
pub fn teardown_loading_screen() {}
