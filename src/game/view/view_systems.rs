use std::f32::consts::PI;

use bevy::camera::Viewport;
use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::game::scenes::scene_in_game::PlayerShip;
use crate::game::scenes::CurrentScene;
use crate::game::view::view_components::*;

pub fn on_window_resized(
    mut messages: MessageReader<WindowResized>,
    camera: Single<&mut Camera, With<GameCamera3d>>,
) {
    let mut camera = camera.into_inner();
    for message in messages.read() {
        camera.viewport = Some(Viewport {
            physical_position: ((0.1 * message.width) as u32, 0).into(),
            physical_size: ((0.8 * message.width) as u32, message.height as u32).into(),
            depth: 0. ..1.,
        });
        info!("Window resized to {} x {}", message.width, message.height);
    }
}

pub fn setup_views(mut commands: Commands) { 
    commands.spawn(ui_camera()); }

pub fn spawn_game_view(mut commands: Commands) {
    const STARTING_POSITION: Vec3 = Vec3::new(0., 0., 10.);
    commands
        .spawn((game_camera3d(STARTING_POSITION), DespawnOnExit(CurrentScene::InGame)))
        .with_child((game_light(), DespawnOnExit(CurrentScene::InGame)));
    commands.spawn((game_camera2d(), DespawnOnExit(CurrentScene::InGame)));
}

pub fn spawn_other_view(mut commands: Commands) {
    commands.spawn((other_light(), DespawnOnEnter(CurrentScene::InGame)));
}

fn game_camera3d(starting_position: Vec3) -> impl Bundle {
    (
        Name::new("GameCamera3D"),
        GameCamera3d,
        Camera3d::default(),
        Camera {
            order: 0,
            clear_color: Color::linear_rgb(0.1, 0.1, 0.2).into(),
            ..Default::default()
        },
        CameraZoom {
            zoom_range:  10. ..100.,
            zoom_speed:  2.,
            zoom_factor: 0.1,
            zoom_target: starting_position.z,
        },
        Projection::Perspective(PerspectiveProjection {
            fov: PI / 4.,
            ..Default::default()
        }),
        Transform::from_translation(starting_position),
    )
}

fn game_camera2d() -> impl Bundle {
    (
        Name::new("GameCamera2D"),
        GameCamera2d,
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 5.),
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
            illuminance: 5_000.,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 0.).looking_at((0., 0., 0.).into(), Vec3::Y),
    )
}

fn other_light() -> impl Bundle {
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
    camera_query: Single<(&mut Transform, &CameraZoom), (With<GameCamera3d>, Without<PlayerShip>)>,
    ship_query: Single<&Transform, With<PlayerShip>>,
) {
    let (mut camera_transform, camera_zoom) = camera_query.into_inner();
    let ship = ship_query.into_inner();
    let target = ship.translation.truncate().extend(camera_zoom.zoom_target);
    camera_transform.translation.smooth_nudge(&target, camera_zoom.zoom_speed, time.delta_secs());
}
