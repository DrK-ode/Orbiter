use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::text::FontSmoothing;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::game::scenes::CurrentScene;

#[derive(Default)]
pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 32.0,
                    font: default(),
                    font_smoothing: FontSmoothing::AntiAliased,
                    ..default()
                },
                text_color: Color::WHITE,
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: true,
                    min_fps: 30.0,
                    target_fps: 60.0,
                },
            },
        })
        .add_plugins(EguiPlugin::default())
        .add_plugins(
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::KeyI)),
        )
        .add_systems(
            Update,
            (
                log_state_change_request.run_if(resource_changed::<NextState<CurrentScene>>),
                log_state_change.run_if(state_changed::<CurrentScene>),
            )
        );
    }
}

pub fn log_state_change_request(state: Res<NextState<CurrentScene>>) {
    match state.into_inner() {
        NextState::Unchanged => {},
        NextState::Pending(state) => {
            info!("State requested to change to {:#?}.", state);
        },
    };
}

pub fn log_state_change(state: Res<State<CurrentScene>>) {
    info!("State changed to {:#?}", **state);
}