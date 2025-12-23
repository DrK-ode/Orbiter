use bevy::prelude::*;
use super::force_components::*;

pub fn calc_gravitational_force(bodies: Query<(&mut ForceAndInertia, &Mass)>) {
    const GRAVITATIONAL_FIELD: Vec2 = Vec2::new(0., 0.);
    for (mut force, mass) in bodies {
        let mass = match mass {
            Mass::Minor(m) | Mass::Major(m) => m,
        };
        force.force += GRAVITATIONAL_FIELD * mass;
    }
}

pub fn reset_torque(torque: Query<&mut TorqueAndInertia>) {
    for mut torque in torque {
        torque.torque = Default::default();
    }
}

pub fn reset_force(force: Query<&mut ForceAndInertia>) {
    for mut f in force {
        f.force = Default::default();
    }
}