pub mod scene_flash;
pub mod scene_game_over;
pub mod scene_in_game;
pub mod scene_loading;
pub mod scene_title;

use bevy::prelude::*;

use {
    scene_flash::plugin_scene_splash, scene_game_over::plugin_scene_game_over,
    scene_in_game::plugin_scene_in_game, scene_loading::plugin_scene_loading,
    scene_title::plugin_scene_title,
};

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect)]
pub enum CurrentScene {
    #[default]
    Splash,
    Title,
    Loading,
    InGame,
    GameOver,
}

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CurrentScene>().add_plugins((
            plugin_scene_splash,
            plugin_scene_title,
            plugin_scene_loading,
            plugin_scene_in_game,
            plugin_scene_game_over,
        ));
    }
}
