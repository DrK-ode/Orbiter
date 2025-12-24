use bevy::prelude::*;

use crate::game::{game_states::ScreenState, view::view_components::*};

pub fn spawn_game_view(mut commands: Commands) {
    commands.spawn((game_camera(), DespawnOnExit(ScreenState::InGame)));
    commands.spawn((game_light(), DespawnOnExit(ScreenState::InGame)));
}

fn game_camera() -> impl Bundle {
    (Name::new("GameCamera"), PlayerCamera, Camera3d::default(), Transform::from_xyz(0., 0., 10.))
}

fn game_light() -> impl Bundle {
    (
        Name::new("GameLight"),
        PlayerLight,
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 10_000.,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(2., 2., 10.).looking_at((0., 0., 0.).into(), Vec3::Y),
    )
}

pub fn spawn_other_view(mut commands: Commands) {
    commands.spawn((other_camera(), DespawnOnEnter(ScreenState::InGame)));
    commands.spawn((other_light(), DespawnOnEnter(ScreenState::InGame)));
}

fn other_camera() -> impl Bundle {
    (Name::new("OtherCamera"), Camera2d, Transform::from_xyz(0., 0., 10.))
}

fn other_light() -> impl Bundle {
    (
        Name::new("OtherLight"),
        PointLight {
            color: Color::WHITE,
            intensity: 100_000.,
            range: 100.,
            radius: 0.,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(2., 2., 10.),
    )
}

pub fn follow_player() {}
