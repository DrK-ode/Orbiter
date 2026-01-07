use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::game::{
    assets::asset_resources::SpaceAssets,
    scenes::GameScene,
    view::{view_components::BackgroundCamera, BACKGROUND_LAYER},
    visuals::visuals_components::BackgroundQuad,
};

pub fn spawn_background(mut commands: Commands, space_assets: Res<SpaceAssets>) {
    //
    // Background quad
    //
    commands.spawn((
        Name::new("Background"),
        DespawnOnExit(GameScene::InGame),
        BackgroundQuad,
        RenderLayers::layer(BACKGROUND_LAYER),
        Mesh3d(space_assets.starry_mesh.clone()),
        MeshMaterial3d(space_assets.noisy_material.clone()),
        Transform::default(),
    ));
}

pub fn move_background(
    camera_query: Single<&Transform, (With<BackgroundCamera>, Without<BackgroundQuad>)>,
    background_query: Single<&mut Transform, With<BackgroundQuad>>,
) {
    let camera_transform = camera_query.into_inner();
    let mut background_transform = background_query.into_inner();
    // Due to the orthographic projection the Z-value will not, per se, affect the visual
    // representation. By encoding the camera distance in teh Z-channel we can use that in the
    // shader to change the level of detail.
    background_transform.translation = camera_transform.translation;
}
