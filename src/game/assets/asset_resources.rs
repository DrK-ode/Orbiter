use bevy::{asset::RenderAssetUsages, prelude::*, render::storage::ShaderStorageBuffer};
use noiz::prelude::*;

use crate::game::visuals::shader_background::{NoisyMaterial, StarryMaterial};

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
    #[dependency]
    pub noisy_material:  Handle<NoisyMaterial>,
}

impl FromWorld for SpaceAssets {
    fn from_world(world: &mut World) -> Self {
        world.resource_scope(|_world, asset_server: Mut<AssetServer>| {
            let mars_scene =
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/mars.glb"));
            let starry_mesh = asset_server.add(Rectangle::new(1., 1.).into());
            let starry_material = asset_server.add(StarryMaterial {});
            let mut noise = Noise::<PerCell<SimplexGrid, Random<UNorm, f32>>>::default();
            noise.set_seed(1234);
            let width = 1000;
            let height = 1000;
            let mut data: Vec<f32> = Vec::with_capacity(width * height);
            for y in 0..height {
                for x in 0..width {
                    let value = noise.sample_for::<f32>(Vec2::new(x as f32 / 10., y as f32 / 10.));
                    // let value = if x > 49 { 255u8 } else { 0u8 };
                    data.push(value);
                }
            }
            let mut data = ShaderStorageBuffer::from(data);
            data.asset_usage = RenderAssetUsages::RENDER_WORLD;
            let data = asset_server.add(data);
            let noisy_material = asset_server.add(NoisyMaterial {
                noise_data: data.clone(),
                noise_width: width as u32,
                noise_height: height as u32,
                star_color: LinearRgba::GREEN,
            });
            Self {
                mars_scene,
                starry_mesh,
                starry_material,
                noisy_material,
            }
        })
    }
}
