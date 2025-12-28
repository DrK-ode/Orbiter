use bevy::prelude::*;

use crate::game::scenes::CurrentScene;

pub fn plugin_scene_game_over(app: &mut App) {
    app.add_systems(OnEnter(CurrentScene::GameOver), spawn_game_over_screen)
        .add_systems(OnExit(CurrentScene::GameOver), teardown_game_over_screen)
        .add_systems(Update, transition_from_game_over.run_if(in_state(CurrentScene::GameOver)));
}

pub fn spawn_game_over_screen() {}
pub fn transition_from_game_over(mut next_state: ResMut<NextState<CurrentScene>>) {
    next_state.set(CurrentScene::Title);
}
pub fn teardown_game_over_screen() {}
