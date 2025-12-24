use bevy::prelude::*;

use super::game_states::{InGameState, ScreenState};
use super::physics::motion::motion_components::{
    Acceleration, AngularVelocity, Direction, Position, Velocity,
};
use super::physics::physics_types::ValueLimit;
use super::{
    assets::asset_resources::{MaterialType, Materials, MeshType, Meshes},
    game_components::PlayerShip,
    input::input_components::{PlayerReticle, TargetDirection},
    physics::force::force_components::ForceAndInertia,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSystemSet {
    Physics,
    Game,
    Menu,
}

pub fn spawn_splash_screen() {}
pub fn transition_from_splash_to_title(mut next_state: ResMut<NextState<ScreenState>>) {
    next_state.set(ScreenState::Title);
}
pub fn teardown_splash_screen() {}

pub fn spawn_title_screen() {}
pub fn transition_from_title_to_loading(mut next_state: ResMut<NextState<ScreenState>>) {
    next_state.set(ScreenState::Loading);
}
pub fn teardown_title_screen() {}

pub fn spawn_loading_screen() {}
pub fn transition_from_loading_to_in_game(mut next_state: ResMut<NextState<ScreenState>>) {
    next_state.set(ScreenState::InGame);
}
pub fn teardown_loading_screen() {}

pub fn spawn_in_game_screen(
    mut commands: Commands,
    meshes: Res<Meshes>,
    materials: Res<Materials>,
) {
    commands.spawn(player_ship(&meshes, &materials));
    commands.spawn(reticle(&meshes, &materials));
}
pub fn transition_from_in_game_to_game_over(
    mut next_state: ResMut<NextState<ScreenState>>,
    sub_state: Res<State<InGameState>>,
) {
    if let InGameState::Quit = sub_state.get() {
        next_state.set(ScreenState::GameOver);
    };
}
pub fn teardown_in_game_screen() {}

pub fn spawn_game_over_screen() {}
pub fn transition_from_game_over_to_title(mut next_state: ResMut<NextState<ScreenState>>) {
    next_state.set(ScreenState::Title);
}
pub fn teardown_game_over_screen() {}

pub fn log_state_change_request(state: Res<NextState<ScreenState>>) {
    match state.into_inner() {
        NextState::Unchanged => {},
        NextState::Pending(state) => {
            info!("State requested to change to {:#?}.", state);
        },
    };
}

pub fn log_state_change(state: Res<State<ScreenState>>) {
    info!("State changed to {:#?}", **state);
}

fn player_ship(meshes: &Res<Meshes>, materials: &Res<Materials>) -> impl Bundle {
    (
        Name::new("Player"),
        DespawnOnExit(ScreenState::InGame),
        PlayerShip {
            thrust: 1.,
            reorient_mode: Default::default(),
        },
        Mesh3d(meshes.get(MeshType::PlayerShip)),
        MeshMaterial3d(materials.get(MaterialType::PlayerShip)),
        Transform::from_xyz(0., 0., 0.),
        Position::new((0., 0.).into()),
        Velocity::default(),
        ValueLimit::<Velocity>::new(1.),
        Acceleration::default(),
        ForceAndInertia::new(1.),
        Direction::new(Dir2::Y),
        TargetDirection::new(Dir2::Y),
        AngularVelocity::default(),
        ValueLimit::<AngularVelocity>::new(2.),
    )
}

fn reticle(meshes: &Res<Meshes>, materials: &Res<Materials>) -> impl Bundle {
    (
        Name::new("PlayerReticle"),
        DespawnOnExit(ScreenState::InGame),
        PlayerReticle,
        Mesh3d(meshes.get(MeshType::Crosshair)),
        MeshMaterial3d(materials.get(MaterialType::Crosshair)),
        Transform::from_xyz(0., 0., 0.1),
        Position::new((0., 0.).into()),
        Velocity::default(),
    )
}
