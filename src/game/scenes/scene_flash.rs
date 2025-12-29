use bevy::prelude::*;

use crate::game::scenes::GameScene;

pub fn plugin_scene_splash(app: &mut App) {
    app.add_systems(OnEnter(GameScene::Splash), spawn_splash_screen)
        .add_systems(OnExit(GameScene::Splash), teardown_splash_screen)
        .add_systems(Update, transition_from_splash.run_if(in_state(GameScene::Splash)));
}

pub fn spawn_splash_screen() {}
pub fn transition_from_splash(mut next_state: ResMut<NextState<GameScene>>) {
    next_state.set(GameScene::Title);
}
pub fn teardown_splash_screen() {}
