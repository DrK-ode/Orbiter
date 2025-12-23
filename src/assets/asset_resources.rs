use bevy::prelude::*;
use bevy::{
    color::palettes::css::{GREEN, RED},
    pbr::StandardMaterial,
};

use enum_collections::{EnumMap, Enumerated, em};

#[derive(Clone, Enumerated)]
pub enum MeshType {
    PlayerShip,
    Planet,
}

#[derive(Clone, Enumerated)]
pub enum MaterialType {
    PlayerShip,
    Planet,
}

#[derive(Resource)]
pub struct ResourceMap<T: Enumerated, U: Asset, const N: usize> {
    resources: EnumMap<T, Handle<U>, N>,
}

pub type Meshes = ResourceMap<MeshType, Mesh, { MeshType::SIZE }>;
pub type Materials = ResourceMap<MaterialType, StandardMaterial, { MaterialType::SIZE }>;

impl<T: Enumerated, U: Asset, const N: usize> ResourceMap<T, U, N> {
    pub fn get(&self, resource_type: T) -> Handle<U> {
        self.resources[resource_type].clone()
    }
}

impl FromWorld for Meshes {
    fn from_world(world: &mut World) -> Self {
        if let Some(mut mesh_assets) = world.get_resource_mut::<Assets<Mesh>>() {
            Self {
                resources: em! (MeshType, Handle<Mesh>,
                   PlayerShip => mesh_assets.add(Triangle2d::new( (1.,0.).into(), (-1.,-0.5).into(), (-1.,0.5).into())),
                   Planet => mesh_assets.add(Sphere::new(1.))
                ),
            }
        } else {
            panic!("No Assets<Mesh> available.");
        }
    }
}

impl FromWorld for Materials {
    fn from_world(world: &mut World) -> Self {
        if let Some(mut material_assets) = world.get_resource_mut::<Assets<StandardMaterial>>() {
            Self {
                resources: em!(MaterialType, Handle<StandardMaterial>,
                   PlayerShip=> material_assets.add(StandardMaterial::from_color(RED)),
                   Planet=> material_assets.add(StandardMaterial::from_color(GREEN))
                ),
            }
        } else {
            panic!("No Assets<StandardMaterial> available.");
        }
    }
}
