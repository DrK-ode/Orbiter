pub mod input_actions;
pub mod input_components;
pub mod input_systems;

use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use input_actions::*;
use input_systems::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputSystems{
    InGame,
    Menu,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AimInput>()
            .add_plugins(InputManagerPlugin::<ShipAction>::default())
            .add_plugins(InputManagerPlugin::<NavigationAction>::default())
            .add_systems(Startup, setup_input)
            .add_systems(
                FixedUpdate,
                (
                    keyboard_aim.run_if(in_state(AimInput::Keyboard)),
                    mouse_aim.run_if(in_state(AimInput::Mouse)),
                    set_ship_course,
                    seek_target_direction,
                    controller_ship_thrust,
                )
                    .chain()
                    .in_set(InputSystems::InGame),
            );
    }
}
