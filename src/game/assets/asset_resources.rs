use bevy::prelude::*;

use crate::game::visuals::materials::BackgroundMaterial;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub ship_scene:    Handle<Scene>,
    #[dependency]
    pub reticle_image: Handle<Image>,
    #[dependency]
    pub speedometer_image: Handle<Image>,
    #[dependency]
    pub speedometer_needle_image: Handle<Image>,
    #[dependency]
    pub compass_needle_image: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        world.resource_scope(|_world, asset_server: Mut<AssetServer>| {
            let ship_scene =
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/ship.glb"));
            let reticle_image = asset_server.load("images/reticle.png");
            let speedometer_image = asset_server.load("images/speedometer.png");
            let speedometer_needle_image = asset_server.load("images/speedometer_needle.png");
            let compass_needle_image = asset_server.load("images/compass_needle.png");
            Self {
                ship_scene,
                reticle_image,
                speedometer_image,
                speedometer_needle_image,
                compass_needle_image,
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
            });
            Self {
                mars_scene,
                starry_mesh,
                noisy_material,
            }
        })
    }
}
