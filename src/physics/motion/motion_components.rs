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
pub struct AngularDirection {
    property: AngleLikeValue,
}

impl AngularDirection {
    pub fn new(angle: f32) -> Self {
        Self {
            property: AngleLikeValue::new_with_buffer(angle),
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
