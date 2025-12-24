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
    property: PropertyValue<Dir2>,
}

impl TargetDirection {
    pub fn new(dir: Dir2) -> Self {
        Self { property: PropertyValue::new(dir) }
    }
}
