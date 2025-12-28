use bevy::prelude::*;

use crate::game::scenes::CurrentScene;

pub fn plugin_scene_splash(app: &mut App) {
    app.add_systems(OnEnter(CurrentScene::Splash), spawn_splash_screen)
        .add_systems(OnExit(CurrentScene::Splash), teardown_splash_screen)
        .add_systems(Update, transition_from_splash.run_if(in_state(CurrentScene::Splash)));
}

pub fn spawn_splash_screen() {}
pub fn transition_from_splash(mut next_state: ResMut<NextState<CurrentScene>>) {
    next_state.set(CurrentScene::Title);
}
pub fn teardown_splash_screen() {}
