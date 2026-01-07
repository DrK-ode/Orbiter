use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{
    camera::visibility::RenderLayers,
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    post_process::bloom::Bloom,
    prelude::*,
    render::view::Hdr,
    window::WindowResized,
};

use crate::game::{
    scenes::{scene_in_game::PlayerShip, GameScene},
    view::{view_components::*, BACKGROUND_LAYER, FOREGROUND_LAYER, MAIN_LAYER},
    visuals::visuals_components::BackgroundQuad,
};

pub fn on_window_resized(
    mut messages: MessageReader<WindowResized>,
    background_query: Single<&mut Transform, With<BackgroundQuad>>,
) {
    let mut background_transform = background_query.into_inner();
    for message in messages.read() {
        background_transform.scale = Vec3::new(1.0, message.height / message.width, 1.);
    }
}

pub fn setup_views(mut commands: Commands) {
    //
    // UI camera
    //
    commands.spawn((
        Name::new("UiCamera"),
        IsDefaultUiCamera,
        Camera2d,
        Camera {
            order: 10,
            clear_color: ClearColorConfig::None,
            ..Default::default()
        },
    ));
}

pub fn spawn_game_view(mut commands: Commands) {
    //
    // Main camera
    //
    const MAIN_CAMERA_STARTING_POSITION: Vec3 = Vec3::new(0., 0., 10.);
    commands.spawn((
        Name::new("GameCamera3D"),
        DespawnOnExit(GameScene::InGame),
        MainCamera,
        RenderLayers::layer(MAIN_LAYER),
        Camera3d::default(),
        Camera {
            order: 0,
            clear_color: ClearColorConfig::None,
            ..Default::default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        DebandDither::Enabled,
        CameraZoom {
            zoom_range:  10. ..100.,
            zoom_speed:  2.,
            zoom_factor: 0.1,
            zoom_target: MAIN_CAMERA_STARTING_POSITION.z,
        },
        Projection::Perspective(PerspectiveProjection {
            fov: PI / 4.,
            ..Default::default()
        }),
        Transform::from_translation(MAIN_CAMERA_STARTING_POSITION),
    ));
    commands.spawn((
        //
        // Background camera
        //
        Name::new("Background Camera"),
        DespawnOnExit(GameScene::InGame),
        BackgroundCamera,
        RenderLayers::layer(BACKGROUND_LAYER),
        Camera3d::default(),
        Camera {
            order: -1,
            clear_color: Color::linear_rgb(0.1, 0.1, 0.2).into(),
            ..Default::default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        DebandDither::Enabled,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::FixedHorizontal { viewport_width: 1. },
            ..OrthographicProjection::default_2d()
        }),
    ));
    //
    // Game light
    //
    commands.spawn((
        Name::new("GameLight"),
        DespawnOnExit(GameScene::InGame),
        GameLight,
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 10_000.,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_translation(MAIN_CAMERA_STARTING_POSITION)
            .looking_at((0., 0., 0.).into(), Vec3::Y),
    ));
    //
    // Foreground camera
    //
    commands.spawn((
        Name::new("Foreground Camera"),
        DespawnOnExit(GameScene::InGame),
        ForegroundCamera,
        RenderLayers::layer(FOREGROUND_LAYER),
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..Default::default()
        },
        Hdr,
    ));
}

pub fn spawn_other_view(mut commands: Commands) {
    commands.spawn((
        Name::new("UiLight"),
        DespawnOnEnter(GameScene::InGame),
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
    ));
}

pub fn move_main_camera(
    time: Res<Time>,
    main_camera_query: Single<
        (&mut Transform, &CameraZoom),
        (With<MainCamera>, (Without<PlayerShip>, Without<GameLight>)),
    >,
    light_query: Single<&mut Transform, (With<GameLight>, Without<PlayerShip>)>,
    ship_query: Single<&Position, With<PlayerShip>>,
) {
    let (mut camera_transform, camera_zoom) = main_camera_query.into_inner();
    let mut light_transform = light_query.into_inner();
    let ship_position = ship_query.into_inner();
    let target = ship_position.0.truncate().extend(camera_zoom.zoom_target);
    camera_transform.translation.smooth_nudge(&target, camera_zoom.zoom_speed, time.delta_secs());
    camera_transform.look_at(ship_position.0, Vec3::Y);
    light_transform.translation = ship_position.0;
}

pub fn move_background_camera(
    background_camera_query: Single<&mut Transform, (With<BackgroundCamera>, Without<MainCamera>)>,
    main_camera_query: Single<&Transform, With<MainCamera>>,
) {
    let mut background_camera_transform = background_camera_query.into_inner();
    let main_camera_transform = main_camera_query.into_inner();
    background_camera_transform.translation = main_camera_transform.translation;
}
