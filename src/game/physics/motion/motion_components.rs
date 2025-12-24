use bevy::prelude::*;

use crate::game::physics::physics_types::*;

#[derive(Component, Debug, Default, Deref, DerefMut, Reflect)]
pub struct Position {
    pub property: Buffered<Vec2>,
}

impl Position {
    pub fn new(value: Vec2) -> Self {
        Self {
            property: Buffered::new(value)
        }
    }
}

#[derive(Component, Debug, Default, Deref, DerefMut, Reflect)]
pub struct Velocity {
    property: Vec2,
}

#[derive(Component, Debug, Default, Deref, DerefMut, Reflect)]
pub struct Acceleration {
    property: Vec2,
}

#[derive(Component, Debug, Deref, DerefMut, Reflect)]
pub struct Direction {
    property: Buffered<Dir2>,
}

impl Direction {
    pub fn new(dir: Dir2) -> Self {
        Self {
            property: Buffered::new(dir),
        }
    }
}

#[derive(Component, Debug, Default, Deref, DerefMut, Reflect)]
pub struct AngularVelocity {
    property: f32,
}

#[derive(Component, Debug, Default, Deref, DerefMut, Reflect)]
pub struct AngularAcceleration {
    property: f32,
}
