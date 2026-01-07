use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
    pbr::MaterialPlugin,
    state::condition::in_state,
};
use bevy_shader_utils::ShaderUtilsPlugin;

use crate::game::{
    scenes::GameScene,
    visuals::{
        materials::{BackgroundMaterial},
        visuals_systems::move_background,
    },
};

pub mod materials;
pub mod visuals_components;
pub mod visuals_systems;

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ShaderUtilsPlugin,
            MaterialPlugin::<BackgroundMaterial>::default(),
        ))
        .add_systems(Update, move_background.run_if(in_state(GameScene::InGame)));
    }
}
