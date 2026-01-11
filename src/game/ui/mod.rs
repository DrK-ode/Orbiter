use bevy::prelude::*;

use crate::game::ui::hud::hud_plugin;

pub mod hud;
pub mod menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(hud_plugin);
    }
}
