use bevy::prelude::*;

use super::input::input_components::ReorientMode;

#[derive(Debug, Component, Reflect)]
pub struct PlayerShip {
    pub thrust: f32,
    pub reorient_mode: ReorientMode,
}

impl Default for PlayerShip {
    fn default() -> Self {
        Self {
            thrust: 1.,
            reorient_mode: Default::default(),
        }
    }
}
