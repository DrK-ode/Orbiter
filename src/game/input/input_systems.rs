use std::cmp::Ordering;

use avian3d::prelude::*;
use bevy::{
    prelude::*,
    window::{CursorOptions, PrimaryWindow},
};
use leafwing_input_manager::prelude::*;

use super::{input_actions::*, input_components::*};
use crate::game::{
    scenes::scene_in_game::{PlayerReticle, PlayerShip},
    view::view_components::{CameraZoom, ForegroundCamera, MainCamera},
};

pub fn setup_input(
    mut commands: Commands,
    _cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    let keyboard_pointer = VirtualDPad::arrow_keys();
    commands.init_resource::<ActionState<ShipAction>>();
    let mut input_map = InputMap::new([
        (ShipAction::OrientPrograde, KeyCode::KeyW),
        (ShipAction::OrientRetrograde, KeyCode::KeyS),
    ]);
    input_map.insert(ShipAction::EngageEngine, MouseButton::Right);
    commands.insert_resource(input_map);
    commands.init_resource::<ActionState<NavigationAction>>();
    commands.insert_resource(
        InputMap::new([
            (NavigationAction::Map, KeyCode::KeyM),
            (NavigationAction::ToggleMenu, KeyCode::F10),
        ])
        .with_axis(NavigationAction::Zoom, MouseScrollAxis::Y)
        .with_dual_axis(NavigationAction::KeyboardAim, keyboard_pointer),
    );

    #[cfg(not(debug_assertions))]
    {
        let mut cursor_options = _cursor_options.into_inner();
        _cursor_options.visible = true;
    }
}

pub fn controller_ship_thrust(
    action: Res<ActionState<ShipAction>>,
    player_ship: Single<(Forces, &Rotation, &PlayerShip)>,
) {
    let (mut force, rotation, ship) = player_ship.into_inner();
    let direction = rotation * Vec3::X;
    if action.pressed(&ShipAction::EngageEngine) {
        let engine_force = direction * ship.thrust;
        force.apply_force(engine_force);
    }
}

pub fn mouse_reticle_control(
    aim_query: Single<&mut Position, With<PlayerReticle>>,
    window_query: Single<&Window, With<PrimaryWindow>>,
    camera2d_query: Single<(&Camera, &GlobalTransform), With<ForegroundCamera>>,
) {
    let mut reticle_position = aim_query.into_inner();
    let window = window_query.into_inner();
    let (camera2d, camera2d_transform) = camera2d_query.into_inner();

    if let Some(cursor) = window.cursor_position()
        && let Ok(ray2d) = camera2d.viewport_to_world_2d(camera2d_transform, cursor)
    {
        reticle_position.0 = ray2d.extend(0.);
    }
}

pub fn keyboard_reticle_control(
    navigation_action: Res<ActionState<NavigationAction>>,
    reticle_query: Single<&mut LinearVelocity, With<PlayerReticle>>,
) {
    const AIM_SPEED: f32 = 2.;
    let mut reticle_velocity = reticle_query.into_inner();
    if let Some(dualaxis) = navigation_action.dual_axis_data(&NavigationAction::KeyboardAim) {
        reticle_velocity.0 += AIM_SPEED * dualaxis.fixed_update_pair.extend(0.);
    }
}

pub fn set_ship_course(
    ship_action: Res<ActionState<ShipAction>>,
    reticle_query: Single<&Position, With<PlayerReticle>>,
    ship_query: Single<(
        &mut DirectionTarget,
        &Position,
        &Rotation,
        &LinearVelocity,
        &mut PlayerShip,
    )>,
    camera2d_query: Single<(&Camera, &GlobalTransform), With<ForegroundCamera>>,
    camera3d_query: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let reticle_position = reticle_query.into_inner().0.truncate();
    let (mut direction_target, ship_position, ship_rotation, ship_velocity, mut ship) =
        ship_query.into_inner();
    let ship_position = ship_position.0.truncate();
    let velocity_normalized = ship_velocity.0.truncate().normalize();
    let speed_is_nonzero = velocity_normalized.length_squared() > 0.;
    const AIM_DEADZONE_RADIUS: f32 = 1.;
    let (camera2d, camera2d_transform) = camera2d_query.into_inner();
    let (camera3d, camera3d_transform) = camera3d_query.into_inner();

    let aim_position = convert_2d_to_3d_position(
        camera2d,
        camera2d_transform,
        camera3d,
        camera3d_transform,
        reticle_position,
    );

    (ship.reorient_mode, direction_target.0) =
        if ship_action.pressed(&ShipAction::OrientPrograde) && speed_is_nonzero {
            (ReorientMode::Prograde, Dir2::new_unchecked(velocity_normalized))
        }
        else if ship_action.pressed(&ShipAction::OrientRetrograde) && speed_is_nonzero {
            (ReorientMode::Retrograde, Rot2::from_sin_cos(0., -1.) * Dir2::new_unchecked(velocity_normalized))
        }
        else if let Some(aim_position) = aim_position && aim_position.truncate().distance_squared(ship_position) > AIM_DEADZONE_RADIUS {
            let new_dir = Dir2::new_unchecked((aim_position.truncate() - ship_position).normalize());
            (ReorientMode::Aim, new_dir)
        }
        else {
            let ship_direction =
                Dir2::new_unchecked((ship_rotation.0 * Vec3::X).truncate().normalize());
            (ReorientMode::Free, ship_direction)
        };
}

fn convert_2d_to_3d_position(
    camera2d: &Camera,
    camera2d_transform: &GlobalTransform,
    camera3d: &Camera,
    camera3d_transform: &GlobalTransform,
    position2d: Vec2,
) -> Option<Vec3> {
    camera2d
        .world_to_viewport(camera2d_transform, position2d.extend(0.))
        .ok()
        .and_then(|viewport_position| {
            camera3d.viewport_to_world(camera3d_transform, viewport_position).ok()
        })
        .and_then(|ray| {
            Some(ray).zip(ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Z)))
        }).map(|(ray, distance)| ray.get_point(distance))
}

pub fn seek_target_direction(
    time: Res<Time<Fixed>>,
    directions: Query<(&DirectionTarget, &Rotation, &mut AngularVelocity)>,
) {
    for (dir_target, rotation, mut velocity) in directions {
        velocity.0 = Vec3::Z * (rotation * Vec3::X).truncate().angle_to(dir_target.0.as_vec2())
            / time.delta_secs();
    }
}

pub fn scroll_zoom(
    navigation_action: Res<ActionState<NavigationAction>>,
    query: Single<&mut CameraZoom, With<MainCamera>>,
) {
    if let Some(zoom) = navigation_action.axis_data(&NavigationAction::Zoom) {
        let mut camera_zoom = query.into_inner();
        let zoom = match zoom.fixed_update_value.partial_cmp(&0.) {
            Some(Ordering::Equal) | None => return,
            Some(Ordering::Greater) => 1. - 1. / camera_zoom.zoom_factor,
            Some(Ordering::Less) => 1. + camera_zoom.zoom_factor,
        };
        let z = (camera_zoom.zoom_target * zoom)
            .clamp(camera_zoom.zoom_range.start, camera_zoom.zoom_range.end);
        camera_zoom.zoom_target = z;
    }
}
