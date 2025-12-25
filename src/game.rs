pub mod assets;
pub mod input;
pub mod physics;
pub mod scenes;
pub mod view;

use bevy::app::Plugin;
use bevy::prelude::*;

use assets::AssetsPlugin;
use input::{InputPlugin, InputSystems};
use physics::{PhysicsPlugin, PhysicsSystems};
use scenes::{GameScene, ScenesPlugin};
use view::ViewPlugin;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((AssetsPlugin, ViewPlugin, InputPlugin, PhysicsPlugin, ScenesPlugin))
            .configure_sets(
                FixedUpdate,
                (
                    InputSystems::InGame.run_if(in_state(GameScene::InGame)),
                    PhysicsSystems.run_if(in_state(GameScene::InGame)),
                )
                    .chain(),
            );
    }
}
