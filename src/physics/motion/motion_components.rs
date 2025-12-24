use bevy::prelude::*;

use crate::physics::physics_traits::*;

#[derive(Component, Debug, Deref, DerefMut, Reflect)]
pub struct Position {
    pub property: PropertyValue<Vec2>,
}

impl Position {
    pub fn new(value: Vec2) -> Self {
        Self {
            property: PropertyValue::new_with_buffer(value)
        }
    }
}

#[derive(Component, Debug, Default, Deref, DerefMut, Reflect)]
pub struct Velocity {
    property: PropertyValue<Vec2>,
}

#[derive(Component, Debug, Default, Deref, DerefMut, Reflect)]
pub struct Acceleration {
    property: PropertyValue<Vec2>,
}

#[derive(Component, Debug, Deref, DerefMut, Reflect)]
pub struct Direction {
    property: PropertyValue<Dir2>,
}

impl Direction {
    pub fn new(dir: Dir2) -> Self {
        Self {
            property: PropertyValue::new_with_buffer(dir),
        }
    }
}

#[derive(Component, Debug, Default, Deref, DerefMut, Reflect)]
pub struct AngularVelocity {
    property: PropertyValue<f32>,
}

#[derive(Component, Debug, Default, Deref, DerefMut, Reflect)]
pub struct AngularAcceleration {
    property: PropertyValue<f32>,
}
