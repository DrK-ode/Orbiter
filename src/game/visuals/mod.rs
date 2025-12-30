use bevy::{
    app::{App, Plugin},
    pbr::MaterialPlugin,
};

use crate::game::visuals::shader_background::{NoisyMaterial, StarryMaterial};

pub mod shader_background;

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialPlugin::<StarryMaterial>::default(),
            MaterialPlugin::<NoisyMaterial>::default(),
        ));
    }
}
