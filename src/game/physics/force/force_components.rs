use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Reflect)]
pub enum Mass {
    Minor(f32), // Does not attract other masses
    Major(f32),
}

#[derive(Component, Default, Debug, Reflect)]
pub struct ForceAndInertia {
    pub force: Vec2,
    pub inertia: f32,
}

impl ForceAndInertia {
    pub fn new(inertia: f32) -> Self {
        Self {
            force: Default::default(),
            inertia,
        }
    }
    pub fn acceleration(&self) -> Option<Vec2> {
        if self.inertia == 0. {
            None
        } else {
            Some(self.force / self.inertia)
        }
    }
}

#[derive(Component, Default, Debug, Reflect)]
pub struct TorqueAndInertia {
    pub torque: f32,
    pub inertia: f32,
}

impl TorqueAndInertia {
    pub fn new(inertia: f32) -> Self {
        Self {
            torque: Default::default(),
            inertia,
        }
    }
    pub fn angular_acceleration(&self) -> Option<f32> {
        if self.inertia == 0. {
            None
        } else {
            Some(self.torque / self.inertia)
        }
    }
}