use bevy::{
    color::palettes::css::{YELLOW},
    prelude::*,
};

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub ship_scene: Handle<Scene>,
    pub reticle_mesh: Handle<Mesh>,
    pub reticle_material: Handle<StandardMaterial>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        world.resource_scope(|world, asset_server: Mut<AssetServer>| {
            world.resource_scope(|world, mut mesh_assets: Mut<Assets<Mesh>>| {
                world.resource_scope(
                    |_world, mut material_assets: Mut<Assets<StandardMaterial>>| {
                        let ship_scene = asset_server
                            .load(GltfAssetLabel::Scene(0).from_asset("models/ship.glb"));
                        let reticle_mesh = mesh_assets.add(Rhombus::new(0.25, 0.25));
                        let reticle_material =
                            material_assets.add(StandardMaterial::from_color(YELLOW));
                        Self {
                            ship_scene,
                            reticle_mesh,
                            reticle_material,
                        }
                    },
                )
            })
        })
    }
}
