use bevy::app::Plugin;
use bevy::prelude::*;

use super::word_building_systems::*;

#[derive(Default)]
pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, player_setup);
    }
}

