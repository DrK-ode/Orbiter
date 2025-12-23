use std::f32::consts::PI;

use bevy::prelude::*;

use crate::physics::motion::motion_components::{
    Acceleration, AngularDirection, AngularVelocity, Position, Velocity,
};
use crate::physics::physics_traits::ValueLimit;
use crate::{
    assets::asset_resources::{MaterialType, Materials, MeshType, Meshes},
    game_world::game_world_components::PlayerShip,
    physics::{
        force::force_components::ForceAndInertia,
        input::input_components::{PlayerAim, TargetDirection},
    },
};

pub fn player_setup(mut commands: Commands, meshes: Res<Meshes>, materials: Res<Materials>) {
    let mesh = meshes.get(MeshType::PlayerShip);
    let material = materials.get(MaterialType::PlayerShip);
    commands.spawn((
        Name::new("Player"),
        PlayerShip {
            thrust: 1.,
            reorient_mode: Default::default(),
        },
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0., 0., 0.),
        Position::new((0., 0.).into()),
        Velocity::default(),
        ValueLimit::<Velocity>::new(1.),
        Acceleration::default(),
        ForceAndInertia::new(1.),
        AngularDirection::new(PI / 2.),
        TargetDirection::new(PI / 2.),
        AngularVelocity::default(),
        ValueLimit::<AngularVelocity>::new(2.),
    ));
    commands.spawn((
        Name::new("PlayerAim"),
        PlayerAim,
        Position::new((0., 0.).into()),
        Velocity::default(),
    ));
}
