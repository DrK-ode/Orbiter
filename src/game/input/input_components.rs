use bevy::prelude::*;

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
pub struct DirectionTarget(pub Dir2);
