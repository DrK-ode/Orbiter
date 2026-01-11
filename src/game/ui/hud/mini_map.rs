use avian3d::prelude::*;
use bevy::prelude::*;

use crate::game::{
    assets::asset_resources::PlayerAssets,
    scenes::{scene_in_game::PlayerShip, GameScene},
};

#[derive(Component, Debug, Reflect)]
pub struct CompassNeedle;

pub fn spawn_mini_map(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    commands.spawn((
        Name::new("MiniMapContainer"),
        DespawnOnExit(GameScene::InGame),
        Node {
            position_type: PositionType::Absolute,
            right: px(0),
            top: px(0),
            width: vw(10.),
            aspect_ratio: Some(1.),
            padding: UiRect::all(px(10.)),
            ..Default::default()
        },
        children![(CompassNeedle, ImageNode::new(player_assets.compass_needle_image.clone()))],
    ));
}

pub fn update_compass(
    needle_query: Single<&mut UiTransform, With<CompassNeedle>>,
    position_query: Single<&Position, With<PlayerShip>>,
) {
    let mut needle_image_node = needle_query.into_inner();
    let position = position_query.into_inner().0.truncate();
    if position == Vec2::ZERO {
        // Angle undefined.
        return;
    }
    // Needle in image is pointing towards +X.
    let angle = position.angle_to(-Vec2::X);
    needle_image_node.rotation = Rot2::radians(angle);
}
