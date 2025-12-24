use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
pub enum AimInput {
    #[default]
    Mouse,
    Keyboard,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
#[actionlike(Button)]
pub enum ShipAction {
    EngageEngine,
    OrientPrograde,
    OrientRetrograde,
    PrimaryWeapon,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
#[actionlike(Button)]
pub enum NavigationAction {
    #[actionlike(DualAxis)]
    KeyboardAim,
    #[actionlike(Axis)]
    Zoom,
    ToggleMenu,
    Map,
}
