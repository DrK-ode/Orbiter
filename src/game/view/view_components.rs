use std::ops::Range;

use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct GameCamera;

#[derive(Component, Debug, Reflect)]
pub struct UiCamera;

#[derive(Component, Debug, Reflect)]
pub struct GameLight;

#[derive(Component, Debug, Reflect)]
pub struct UiLight;

#[derive(Component, Debug, Reflect)]
pub struct CameraZoom {
    pub zoom_range: Range<f32>,
    pub zoom_speed: f32,
    pub zoom_factor: f32,
    pub zoom_target: f32,
}
