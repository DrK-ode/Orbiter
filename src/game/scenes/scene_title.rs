use bevy::prelude::*;

use crate::game::scenes::GameScene;

pub fn plugin_scene_title(app: &mut App) {
    app.add_systems(OnEnter(GameScene::Title), spawn_title_screen)
        .add_systems(OnExit(GameScene::Title), teardown_title_screen)
        .add_systems(Update, transition_from_title.run_if(in_state(GameScene::Title)));
}

pub fn spawn_title_screen() {}
pub fn transition_from_title(mut next_state: ResMut<NextState<GameScene>>) {
    next_state.set(GameScene::Loading);
}
pub fn teardown_title_screen() {}
