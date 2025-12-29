use bevy::prelude::*;

use crate::game::view::shader_background::StarryMaterial;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub ship_scene:    Handle<Scene>,
    #[dependency]
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

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct SpaceAssets {
    #[dependency]
    pub mars_scene:      Handle<Scene>,
    #[dependency]
    pub starry_mesh:     Handle<Mesh>,
    #[dependency]
    pub starry_material: Handle<StarryMaterial>,
}

impl FromWorld for SpaceAssets {
    fn from_world(world: &mut World) -> Self {
        world.resource_scope(|_world, asset_server: Mut<AssetServer>| {
            let mars_scene =
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/mars.glb"));
            let starry_mesh = asset_server.add(Rectangle::new(1.,1.).into());
            let starry_material = asset_server.add(StarryMaterial {
                color:   LinearRgba::RED,
                texture: asset_server.load("images/test.png"),
            });
            Self {
                mars_scene,
                starry_mesh,
                starry_material,
            }
        })
    }
}
