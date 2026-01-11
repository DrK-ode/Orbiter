use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    input::common_conditions::input_toggle_active,
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::game::scenes::GameScene;

#[derive(Default)]
pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    enabled: false,
                    frame_time_graph_config: FrameTimeGraphConfig {
                        enabled:    false,
                        min_fps:    30.,
                        target_fps: 60.0,
                    },
                    ..Default::default()
                },
            },
            EguiPlugin::default(),
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::KeyI)),
        ))
        .add_systems(
            Update,
            (
                customize_fps_overlay,
                log_state_change_request.run_if(resource_changed::<NextState<GameScene>>),
                log_state_change.run_if(state_changed::<GameScene>),
            ),
        );
    }
}

pub fn log_state_change_request(state: Res<NextState<GameScene>>) {
    match state.into_inner() {
        NextState::Unchanged => {},
        NextState::Pending(state) => {
            info!("State requested to change to {:#?}.", state);
        },
    };
}

pub fn log_state_change(state: Res<State<GameScene>>) {
    info!("State changed to {:#?}", **state);
}

fn customize_fps_overlay(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::KeyF) {
        overlay.enabled = !overlay.enabled;
    }
    if input.just_released(KeyCode::KeyG) {
        overlay.frame_time_graph_config.enabled = !overlay.frame_time_graph_config.enabled;
    }
}
