use avian3d::prelude::{LinearVelocity, MaxLinearSpeed};
use bevy::{prelude::*, render::render_resource::AsBindGroup};

use crate::game::{
    assets::asset_resources::PlayerAssets,
    scenes::{scene_in_game::PlayerShip, GameScene},
};

#[derive(Component, Debug, Reflect)]
pub struct Speedometer;

#[derive(Component, Debug, Reflect)]
pub struct SpeedText;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct SpeedometerMaterial {
    #[uniform(0)]
    pub relative_speed: f32,
}

impl UiMaterial for SpeedometerMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/speedometer.wgsl".into()
    }
}

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
            Speedometer,
            MaterialNode(player_assets.speedometer_material.clone()),
            Node {
                width: vw(100.),
                aspect_ratio: Some(1.),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            children![
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
    mut materials: ResMut<Assets<SpeedometerMaterial>>,
    material_node_query: Single<&MaterialNode<SpeedometerMaterial>, With<Speedometer>>,
    text_query: Single<&mut Text, With<SpeedText>>,
    velocity_query: Single<(&LinearVelocity, &MaxLinearSpeed), With<PlayerShip>>,
) {
    let material_handle = &material_node_query.into_inner().0;
    let mut text = text_query.into_inner();
    let (velocity, max_speed) = velocity_query.into_inner();
    
    let relative_speed = velocity.length() / max_speed.0;
    if let Some(material) = materials.get_mut(material_handle){
        material.relative_speed = relative_speed;
    }
    text.0 = format!("{:.1}\npc/h", relative_speed);
}
