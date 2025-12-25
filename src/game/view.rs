pub mod view_components;
pub mod view_systems;

use bevy::prelude::*;

use super::scenes::GameScene;
use view_systems::*;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::InGame), spawn_game_view)
            .add_systems(OnExit(GameScene::InGame), spawn_other_view)
            .add_systems(Update, move_camera.run_if(in_state(GameScene::InGame)));
    }
}
