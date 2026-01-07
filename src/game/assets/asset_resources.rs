use bevy::prelude::*;

use crate::game::visuals::materials::BackgroundMaterial;

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
    pub mars_scene:     Handle<Scene>,
    #[dependency]
    pub starry_mesh:    Handle<Mesh>,
    #[dependency]
    pub noisy_material: Handle<BackgroundMaterial>,
}

impl FromWorld for SpaceAssets {
    fn from_world(world: &mut World) -> Self {
        world.resource_scope(|_world, asset_server: Mut<AssetServer>| {
            let mars_scene =
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/mars.glb"));
            let starry_mesh = asset_server.add(Rectangle::new(1., 1.).into());
            let noisy_material = asset_server.add(BackgroundMaterial {
                min_z: 10.,
                max_z: 100.,
                zoom_factor: 0.5,
            });
            Self {
                mars_scene,
                starry_mesh,
                noisy_material,
            }
        })
    }
}
