pub mod asset_resources;

use std::collections::VecDeque;

use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ResourceHandles>();
        app.add_systems(PreUpdate, load_asset_resources);
    }
}

type InsertLoadedResource = fn(&mut World, &UntypedHandle);

#[derive(Resource, Default)]
pub struct ResourceHandles {
    waiting:  VecDeque<(UntypedHandle, InsertLoadedResource)>,
    finished: Vec<UntypedHandle>,
}

impl ResourceHandles {
    pub fn is_all_loaded(&self) -> bool { self.waiting.is_empty() }
}

pub trait AssetResourceLoading {
    fn load_asset_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self;
}

impl AssetResourceLoading for App {
    fn load_asset_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self {
        self.init_asset::<T>();
        let world = self.world_mut();
        let resource_asset = T::from_world(world);
        let asset_server = world.resource_mut::<AssetServer>();
        let handle = asset_server.add(resource_asset);
        let mut handles = world.resource_mut::<ResourceHandles>();
        handles.waiting.push_back((handle.untyped(), |world, handle| {
            let assets = world.resource::<Assets<T>>();
            if let Some(asset_resource) = assets.get(handle.id().typed()){
                world.insert_resource(asset_resource.clone());
            }
        }));
        self
    }
}

fn load_asset_resources(world: &mut World) {
    world.resource_scope(|world, mut resource_handles: Mut<ResourceHandles>|{
        world.resource_scope(|world, asset_server: Mut<AssetServer>| {
            for _ in 0..resource_handles.waiting.len() {
                let (handle, on_insert) = resource_handles.waiting.pop_front().unwrap();
                if asset_server.is_loaded_with_dependencies(&handle){
                    on_insert(world, &handle);
                    resource_handles.finished.push(handle);
                } else {
                    resource_handles.waiting.push_back((handle, on_insert));
                }
            }
        })
    });
}