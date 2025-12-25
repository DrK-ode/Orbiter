use std::f32::consts::PI;

use bevy::prelude::*;

use crate::game::scenes::scene_in_game::PlayerShip;
use crate::game::scenes::GameScene;
use crate::game::view::view_components::*;

pub fn spawn_game_view(mut commands: Commands) {
    commands.spawn((game_camera(), DespawnOnExit(GameScene::InGame)));
    commands.spawn((ui_camera(), DespawnOnExit(GameScene::InGame)));
    commands.spawn((game_light(), DespawnOnExit(GameScene::InGame)));
}

pub fn spawn_other_view(mut commands: Commands) {
    commands.spawn((ui_camera(), DespawnOnEnter(GameScene::InGame)));
    commands.spawn((ui_light(), DespawnOnEnter(GameScene::InGame)));
}

fn game_camera() -> impl Bundle {
    const STARTING_POSITION: Vec3 = Vec3::new(0., 0., 10.);
    (
        Name::new("GameCamera"),
        GameCamera,
        Camera3d::default(),
        CameraZoom {
            zoom_range:  10. ..100.,
            zoom_speed:  2.,
            zoom_factor: 0.1,
            zoom_target: STARTING_POSITION.z,
        },
        Projection::Perspective(PerspectiveProjection {
            fov: PI / 4.,
            ..Default::default()
        }),
        Transform::from_translation(STARTING_POSITION),
    )
}

fn ui_camera() -> impl Bundle {
    (
        Name::new("UiCamera"),
        UiCamera,
        Camera2d,
        Camera {
            order: 1000,
            clear_color: ClearColorConfig::None,
            ..Default::default()
        },
        IsDefaultUiCamera,
        Transform::from_xyz(0., 0., 5.),
    )
}

fn game_light() -> impl Bundle {
    (
        Name::new("GameLight"),
        GameLight,
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 10_000.,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 5.).looking_at((0., 0., 0.).into(), Vec3::Y),
    )
}

fn ui_light() -> impl Bundle {
    (
        Name::new("UiLight"),
        UiLight,
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

pub fn move_camera(
    time: Res<Time>,
    camera_query: Single<(&mut Transform, &CameraZoom), (With<GameCamera>, Without<PlayerShip>)>,
    ship_query: Single<&Transform, (With<PlayerShip>, Without<GameCamera>)>,
) {
    let (mut camera_transform, camera_zoom) = camera_query.into_inner();
    let ship = ship_query.into_inner();
    let target = ship.translation.truncate().extend(camera_zoom.zoom_target);
    camera_transform.translation.smooth_nudge(&target, camera_zoom.zoom_speed, time.delta_secs());
}
