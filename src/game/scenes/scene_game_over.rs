use bevy::prelude::*;

use crate::game::scenes::GameScene;

pub fn plugin_scene_game_over(app: &mut App) {
    app.add_systems(OnEnter(GameScene::GameOver), spawn_game_over_screen)
        .add_systems(OnExit(GameScene::GameOver), teardown_game_over_screen)
        .add_systems(Update, transition_from_game_over.run_if(in_state(GameScene::GameOver)));
}

pub fn spawn_game_over_screen() {}
pub fn transition_from_game_over(mut next_state: ResMut<NextState<GameScene>>) {
    next_state.set(GameScene::Title);
}
pub fn teardown_game_over_screen() {}
