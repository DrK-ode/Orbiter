use bevy::prelude::*;

use super::asset_resources::Meshes;
use super::asset_resources::Materials;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Meshes>().init_resource::<Materials>();
    }
}