use bevy::color::palettes::css::YELLOW;
use bevy::prelude::*;
use bevy::{
    color::palettes::css::{GREEN, RED},
    pbr::StandardMaterial,
};

use enum_collections::{em, EnumMap, Enumerated};

#[derive(Clone, Enumerated)]
pub enum MeshType {
    Crosshair,
    Planet,
    PlayerShip,
}

#[derive(Clone, Enumerated)]
pub enum MaterialType {
    Crosshair,
    Planet,
    PlayerShip,
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
                    Crosshair => mesh_assets.add(Rhombus::new(0.25, 0.25)),
                    Planet => mesh_assets.add(Circle::new(5.)),
                    PlayerShip => mesh_assets.add(Triangle2d::new( (1.,0.).into(), (-1.,-0.5).into(), (-1.,0.5).into()))
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
                    Crosshair => material_assets.add(StandardMaterial::from_color(YELLOW)),
                    Planet=> material_assets.add(StandardMaterial::from_color(GREEN)),
                    PlayerShip=> material_assets.add(StandardMaterial::from_color(RED))
                ),
            }
        } else {
            panic!("No Assets<StandardMaterial> available.");
        }
    }
}
