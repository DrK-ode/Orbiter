use bevy::prelude::*;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub ship_scene:    Handle<Scene>,
    pub reticle_image: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        world.resource_scope(|_world, asset_server: Mut<AssetServer>| {
            let reticle_image = asset_server.load("images/reticle.png");
            let ship_scene =
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/ship.glb"));
            Self {
                ship_scene,
                reticle_image,
            }
        })
    }
}
