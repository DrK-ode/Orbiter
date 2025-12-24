
use bevy::{
    prelude::*, window::{CursorOptions, PrimaryWindow}
};
use leafwing_input_manager::prelude::*;

use super::{input_actions::*, input_components::*};
use crate::{game_world::game_world_components::PlayerShip, physics::prelude::*};

pub fn input_setup(
    mut commands: Commands,
    cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
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

    let mut cursor_options = cursor_options.into_inner();
    cursor_options.visible = false;
}

pub fn controller_ship_thrust(
    action: Res<ActionState<ShipAction>>,
    player_ship: Single<(&mut ForceAndInertia, &Direction, &PlayerShip)>,
) {
    let (mut force, direction, ship) = player_ship.into_inner();
    if action.pressed(&ShipAction::EngageEngine) {
        let engine_force = direction.as_vec2() * ship.thrust;
        force.force += engine_force;
    }
}

pub fn mouse_aim(
    aim_query: Single<&mut Position, With<PlayerAim>>,
    window_query: Single<&Window, With<PrimaryWindow>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    let mut aim = aim_query.into_inner();
    let window = window_query.into_inner();
    let (camera, camera_transform) = camera_query.into_inner();

    if let Some(p) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .and_then(|ray| {
            Some(ray)
                .zip(ray.intersect_plane(aim.get_value().extend(0.), InfinitePlane3d::new(Vec3::Z)))
        })
        .map(|(ray, p)| ray.get_point(p))
    {
        aim.set_value(p.xy());
    }
}

pub fn keyboard_aim(
    navigation_action: Res<ActionState<NavigationAction>>,
    aim_query: Single<&mut Velocity, With<PlayerAim>>,
) {
    const AIM_SPEED: f32 = 2.;
    let mut aim_velocity = aim_query.into_inner();
    if let Some(dualaxis) = navigation_action.dual_axis_data(&NavigationAction::KeyboardAim) {
        aim_velocity.add_assign(AIM_SPEED * dualaxis.fixed_update_pair);
    }
}

pub fn set_ship_course(
    ship_action: Res<ActionState<ShipAction>>,
    aim_query: Single<&Position, With<PlayerAim>>,
    ship_query: Single<(
        &mut TargetDirection,
        &Velocity,
        &Position,
        &Direction,
        &mut PlayerShip,
    )>,
) {
    let aim_position = aim_query.into_inner();
    let (mut target_direction, velocity, position, direction, mut ship) = ship_query.into_inner();

    target_direction.set_value(if ship_action.pressed(&ShipAction::OrientPrograde) {
        ship.reorient_mode = ReorientMode::Prograde;
        Dir2::new_unchecked(velocity.normalize())
    } else if ship_action.pressed(&ShipAction::OrientRetrograde) {
        ship.reorient_mode = ReorientMode::Retrograde;
        Rot2::from_sin_cos(0., -1.) * Dir2::new_unchecked(velocity.normalize())
    } else if aim_position.get_value().distance_squared(position.get_value()) > 1. {
        ship.reorient_mode = ReorientMode::Aim;
        Dir2::new_unchecked((aim_position.get_value() - position.get_value()).normalize())
    } else {
        ship.reorient_mode = ReorientMode::Free;
        ***direction
    });
}

pub fn seek_target_direction(
    time: Res<Time<Fixed>>,
    directions: Query<(&TargetDirection, &Direction, &mut AngularVelocity)>,
) {
    for (target, current, mut velocity) in directions {
        velocity.set_value(current.rotation_to(***target).as_radians() / time.delta_secs());
    }
}
