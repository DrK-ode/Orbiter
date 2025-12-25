pub mod force;
pub mod motion;
pub mod physics_types;

pub mod prelude {
    pub use super::{
        force::force_components::*, motion::motion_components::*, physics_types::*, PhysicsPlugin,
    };
}

use bevy::prelude::*;

use force::force_systems::*;
use motion::motion_systems::*;
use prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSystems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.)).add_systems(
            FixedUpdate,
            (
                (calc_gravitational_force).chain(),
                (
                    calc_angular_acceleration,
                    limit_float::<AngularAcceleration>,
                    calc_angular_velocity,
                    limit_float::<AngularVelocity>,
                    calc_direction,
                    calc_acceleration,
                    limit_vec2::<Acceleration>,
                    calc_velocity,
                    limit_vec2::<Velocity>,
                    calc_position,
                )
                    .chain(),
                (reset_force, reset_torque, interpolate_position, interpolate_rotation),
            )
                .chain()
                .in_set(PhysicsSystems),
        );
    }
}
