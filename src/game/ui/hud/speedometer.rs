use std::f32::consts::PI;

use avian3d::prelude::{LinearVelocity, MaxLinearSpeed};
use bevy::prelude::*;

use crate::game::{
    assets::asset_resources::PlayerAssets,
    scenes::{scene_in_game::PlayerShip, GameScene},
};

#[derive(Component, Debug, Reflect)]
pub struct SpeedometerNeedle;

#[derive(Component, Debug, Reflect)]
pub struct SpeedText;

pub fn spawn_speedometer(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    commands.spawn((
        Name::new("SpeedometerContainer"),
        DespawnOnExit(GameScene::InGame),
        Node {
            position_type: PositionType::Absolute,
            left: px(0),
            bottom: px(0),
            width: vw(10.),
            padding: UiRect::all(px(10.)),
            aspect_ratio: Some(1.),
            ..Default::default()
        },
        children![(
            Name::new("Speedometer"),
            ImageNode::new(player_assets.speedometer_image.clone()),
            Node {
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            children![
                (
                    Name::new("SpeedometerNeedle"),
                    SpeedometerNeedle,
                    ImageNode::new(player_assets.speedometer_needle_image.clone())
                ),
                (
                    Name::new("SpeedText"),
                    SpeedText,
                    Node {
                        position_type: PositionType::Absolute,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    Text::default(),
                    TextLayout::new(Justify::Center, LineBreak::WordBoundary),
                    TextFont::from_font_size(14.0),
                    TextColor::WHITE
                ),
            ]
        )],
    ));
}

pub fn update_speedometer(
    needle_query: Single<&mut UiTransform, With<SpeedometerNeedle>>,
    text_query: Single<&mut Text, With<SpeedText>>,
    velocity_query: Single<(&LinearVelocity, &MaxLinearSpeed), With<PlayerShip>>,
) {
    let mut needle_image_node = needle_query.into_inner();
    let mut text = text_query.into_inner();
    let (velocity, max_speed) = velocity_query.into_inner();
    let speed = velocity.length();
    text.0 = format!("{:.1}\npc/h", speed);
    let angle = speed / max_speed.0 * 14. / 9. * PI;
    needle_image_node.rotation = Rot2::radians(angle);
}
