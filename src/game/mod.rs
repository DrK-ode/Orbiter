pub mod assets;
pub mod gravity;
pub mod input;
pub mod scenes;
pub mod ui;
pub mod util;
pub mod view;
pub mod visuals;

use avian3d::prelude::*;
use bevy::prelude::*;

use input::InputSystems;
use scenes::GameScene;

use crate::game::gravity::GravitySystems;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            // ForcePlugin,
            PhysicsPlugins::default(),
            gravity::GravityPlugin::default(),
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
            (
                InputSystems::InGame.run_if(in_state(GameScene::InGame)),
                GravitySystems.run_if(in_state(GameScene::InGame)),
            ),
        );
    }
}
