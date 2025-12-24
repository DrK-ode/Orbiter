use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use super::super::input::input_actions::*;
use super::super::input::input_systems::*;
use super::{force::force_systems::*, motion::motion_systems::*, prelude::*};
use crate::game::game_systems::GameSystemSet;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.))
            .init_state::<AimInput>()
            .add_plugins(InputManagerPlugin::<ShipAction>::default())
            .add_plugins(InputManagerPlugin::<NavigationAction>::default())
            .add_systems(Startup, setup_input)
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
                    .in_set(GameSystemSet::Physics),
            );
    }
}
