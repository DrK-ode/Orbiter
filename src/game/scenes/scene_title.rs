use bevy::prelude::*;

use crate::game::scenes::CurrentScene;

pub fn plugin_scene_title(app: &mut App) {
    app.add_systems(OnEnter(CurrentScene::Title), spawn_title_screen)
        .add_systems(OnExit(CurrentScene::Title), teardown_title_screen)
        .add_systems(Update, transition_from_title.run_if(in_state(CurrentScene::Title)));
}

pub fn spawn_title_screen() {}
pub fn transition_from_title(mut next_state: ResMut<NextState<CurrentScene>>) {
    next_state.set(CurrentScene::Loading);
}
pub fn teardown_title_screen() {}
