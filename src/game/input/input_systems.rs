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
    view::view_components::{CameraZoom, GameCamera3d},
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

pub fn mouse_aim(
    aim_query: Single<&mut Position, With<PlayerReticle>>,
    window_query: Single<&Window, With<PrimaryWindow>>,
    camera_query: Single<(&Camera, &GlobalTransform), With<GameCamera3d>>,
) {
    let mut aim_position = aim_query.into_inner();
    let window = window_query.into_inner();
    let (camera, camera_transform) = camera_query.into_inner();

    if let Some(p) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .and_then(|ray| {
            Some(ray).zip(ray.intersect_plane(aim_position.0, InfinitePlane3d::new(Vec3::Z)))
        })
        .map(|(ray, p)| ray.get_point(p))
    {
        aim_position.0 = p.xy().extend(0.);
    }
}

pub fn keyboard_aim(
    navigation_action: Res<ActionState<NavigationAction>>,
    aim_query: Single<&mut LinearVelocity, With<PlayerReticle>>,
) {
    const AIM_SPEED: f32 = 2.;
    let mut aim_velocity = aim_query.into_inner();
    if let Some(dualaxis) = navigation_action.dual_axis_data(&NavigationAction::KeyboardAim) {
        aim_velocity.0 += AIM_SPEED * dualaxis.fixed_update_pair.extend(0.);
    }
}

pub fn set_ship_course(
    ship_action: Res<ActionState<ShipAction>>,
    aim_query: Single<&Position, With<PlayerReticle>>,
    ship_query: Single<(
        &mut DirectionTarget,
        &Position,
        &Rotation,
        &LinearVelocity,
        &mut PlayerShip,
    )>,
) {
    let aim_position = aim_query.into_inner().0.truncate();
    let (mut direction_target, ship_position, ship_rotation, ship_velocity, mut ship) =
        ship_query.into_inner();
    let ship_position = ship_position.0.truncate();
    let velocity = ship_velocity.0.truncate().normalize();
    let speed_is_nonzero = velocity.length_squared() > 0.;
    const AIM_DEADZONE_RADIUS: f32 = 1.;

    (ship.reorient_mode, direction_target.0) =
        if ship_action.pressed(&ShipAction::OrientPrograde) && speed_is_nonzero {
            (ReorientMode::Prograde, Dir2::new_unchecked(velocity))
        }
        else if ship_action.pressed(&ShipAction::OrientRetrograde) && speed_is_nonzero {
            (ReorientMode::Retrograde, Rot2::from_sin_cos(0., -1.) * Dir2::new_unchecked(velocity))
        }
        else if aim_position.distance_squared(ship_position) > AIM_DEADZONE_RADIUS {
            let new_dir = Dir2::new_unchecked((aim_position - ship_position).normalize());
            (ReorientMode::Aim, new_dir)
        }
        else {
            let ship_direction =
                Dir2::new_unchecked((ship_rotation.0 * Vec3::X).truncate().normalize());
            (ReorientMode::Free, ship_direction)
        };
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
    query: Single<&mut CameraZoom, With<GameCamera3d>>,
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
