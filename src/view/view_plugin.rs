use bevy::prelude::*;

use super::view_systems::*;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, view_setup)
            .add_systems(Update, follow_player);
    }
}
