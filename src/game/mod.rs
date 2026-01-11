pub mod assets;
pub mod input;
pub mod scenes;
pub mod view;
pub mod visuals;
pub mod ui;

use avian3d::prelude::*;
use bevy::prelude::*;

use input::InputSystems;
use scenes::GameScene;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            avian3d::PhysicsPlugins::default(),
            assets::AssetsPlugin,
            visuals::VisualsPlugin,
            view::ViewPlugin,
            ui::UiPlugin,
            input::InputPlugin,
            scenes::ScenesPlugin,
        ))
        .insert_resource(Gravity::ZERO)
        .configure_sets(
            FixedUpdate,
            (InputSystems::InGame.run_if(in_state(GameScene::InGame)),).chain(),
        );
    }
}
