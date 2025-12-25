pub mod asset_resources;

use bevy::prelude::*;

use asset_resources::Meshes;
use asset_resources::Materials;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Meshes>().init_resource::<Materials>();
    }
}