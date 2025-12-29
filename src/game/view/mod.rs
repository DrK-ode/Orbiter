pub mod shader_background;
pub mod view_components;
pub mod view_systems;

use bevy::{prelude::*, sprite_render::Material2dPlugin, window::WindowResized};

use crate::game::view::shader_background::StarryMaterial;

use super::scenes::GameScene;
use view_systems::*;

pub const CAMERA_3D_LAYER: usize = 0;
pub const CAMERA_2D_FOREGROUND_LAYER: usize = 1;
pub const CAMERA_2D_BACKGROUND_LAYER: usize = 2;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<StarryMaterial>::default());
        app.add_systems(Startup, setup_views)
            .add_systems(OnEnter(GameScene::InGame), spawn_game_view)
            .add_systems(OnExit(GameScene::InGame), spawn_other_view)
            .add_systems(PreUpdate, on_window_resized.run_if(on_message::<WindowResized>))
            .add_systems(FixedUpdate, move_camera.run_if(in_state(GameScene::InGame)));
    }
}
