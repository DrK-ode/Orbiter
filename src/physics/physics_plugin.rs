use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use super::{
    force::force_systems::*, input::input_actions::*, input::input_systems::*,
    motion::motion_systems::*, prelude::*,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AimInput>()
            .add_plugins(InputManagerPlugin::<ShipAction>::default())
            .add_plugins(InputManagerPlugin::<NavigationAction>::default())
            .add_systems(Startup, input_setup)
            .add_systems(
                FixedUpdate,
                (
                    (
                        keyboard_aim.run_if(in_state(AimInput::Keyboard)),
                        mouse_aim.run_if(in_state(AimInput::Mouse)),
                        set_ship_course,
                        seek_target_direction,
                        controller_ship_thrust,
                    )
                        .chain(),
                    calc_gravitational_force,
                    ((
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
                        .chain(),),
                )
                    .chain(),
            )
            .add_systems(
                FixedPostUpdate,
                (reset_force, reset_torque, interpolate_position, interpolate_rotation),
            );
    }
}
