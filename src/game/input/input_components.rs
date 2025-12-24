use bevy::prelude::*;

use crate::game::physics::physics_types::*;

#[derive(Debug, Component)]
pub struct PlayerReticle;

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
    property: Buffered<Dir2>,
}

impl TargetDirection {
    pub fn new(dir: Dir2) -> Self {
        Self {
            property: Buffered::new(dir),
        }
    }
}
