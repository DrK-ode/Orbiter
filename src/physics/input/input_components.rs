use bevy::prelude::*;

use crate::physics::physics_traits::*;

#[derive(Debug, Component)]
pub struct PlayerAim;

#[derive(Debug, Default, Eq, PartialEq, Reflect)]
pub enum ReorientMode {
    #[default]
    Free,
    Aim,
    Prograde,
    Retrograde,
    Strafe,
}

#[derive(Component, Debug, Deref, DerefMut, Reflect)]
pub struct TargetDirection {
    property: AngleLikeValue,
}

impl TargetDirection {
    pub fn new(angle: f32) -> Self {
        Self { property: AngleLikeValue::new(angle) }
    }
}
