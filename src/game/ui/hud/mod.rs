use bevy::prelude::*;

use crate::game::{
    scenes::GameScene,
    ui::hud::{
        mini_map::{spawn_mini_map, update_compass},
        speedometer::update_speedometer,
    },
};
use speedometer::spawn_speedometer;

pub mod mini_map;
pub mod speedometer;

pub fn hud_plugin(app: &mut bevy::app::App) {
    app.add_systems(OnEnter(GameScene::InGame), (spawn_speedometer, spawn_mini_map)).add_systems(
        Update,
        (
            update_speedometer.run_if(in_state(GameScene::InGame)),
            update_compass.run_if(in_state(GameScene::InGame)),
        ),
    );
}
