pub mod view_components;
pub mod view_systems;

use bevy::prelude::*;
use bevy::window::WindowResized;

use super::scenes::CurrentScene;
use view_systems::*;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_views)
            .add_systems(OnEnter(CurrentScene::InGame), spawn_game_view)
            .add_systems(OnExit(CurrentScene::InGame), spawn_other_view)
            .add_systems(PreUpdate, on_window_resized.run_if(on_message::<WindowResized>))
            .add_systems(FixedUpdate, move_camera.run_if(in_state(CurrentScene::InGame)));
    }
}
