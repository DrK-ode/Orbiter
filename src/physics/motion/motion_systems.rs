use std::ops::DerefMut;

use bevy::{ecs::component::Mutable, prelude::*};

use crate::physics::prelude::*;

pub fn calc_acceleration(bodies: Query<(&mut Acceleration, &ForceAndInertia)>) {
    for (mut acceleration, force) in bodies {
        if let Some(a) = force.acceleration() {
            acceleration.set_value(a);
        }
    }
}

pub fn calc_velocity(time: Res<Time>, bodies: Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, acceleration) in bodies {
        velocity.add_assign(acceleration.get_value() * time.delta_secs());
    }
}

pub fn calc_position(time: Res<Time>, bodies: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in bodies {
        position.add_assign(velocity.get_value() * time.delta_secs());
    }
}

pub fn interpolate_position(time: Res<Time<Fixed>>, positions: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in positions {
        let interpolated = position.estimate(time.overstep_fraction());
        transform.translation = interpolated.extend(0.);
    }
}

pub fn calc_angular_acceleration(bodies: Query<(&mut AngularAcceleration, &TorqueAndInertia)>) {
    for (mut angular_acceleration, torque) in bodies {
        if let Some(a) = torque.angular_acceleration() {
            angular_acceleration.set_value(a);
        }
    }
}

pub fn calc_angular_velocity(
    time: Res<Time>,
    bodies: Query<(&mut AngularVelocity, &AngularAcceleration)>,
) {
    for (mut angular_velocity, angular_acceleration) in bodies {
        angular_velocity.add_assign(angular_acceleration.get_value() * time.delta_secs());
    }
}

pub fn calc_direction(time: Res<Time>, bodies: Query<(&mut AngularDirection, &AngularVelocity)>) {
    for (mut direction, angular_velocity) in bodies {
        direction.rotate(angular_velocity.get_value() * time.delta_secs());
    }
}

pub fn interpolate_rotation(
    time: Res<Time<Fixed>>,
    directions: Query<(&AngularDirection, &mut Transform)>,
) {
    for (direction, mut transform) in directions {
        transform.rotation = Quat::from_rotation_z(direction.estimate(time.overstep_fraction()));
    }
}

pub fn limit_float<U: DerefMut<Target = PropertyValue<f32>> + Component<Mutability = Mutable>>(
    query: Query<(&mut U, &ValueLimit<U>)>,
) {
    for (mut value, value_max) in query {
        let temp = value.get_value().clamp(-value_max.limit, value_max.limit);
        value.set_value(temp);
    }
}

pub fn limit_vec2<U: DerefMut<Target = PropertyValue<Vec2>> + Component<Mutability = Mutable>>(
    query: Query<(&mut U, &ValueLimit<U>)>,
) {
    for (mut value, value_max) in query {
        let temp = value.get_value().clamp_length_max(value_max.limit);
        value.set_value(temp);
    }
}
