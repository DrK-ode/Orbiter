use bevy::prelude::*;

use crate::game::game_states::ScreenState;

use super::view_systems::*;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ScreenState::InGame), spawn_game_view)
            .add_systems(OnExit(ScreenState::InGame), spawn_other_view)
            .add_systems(Update, follow_player.run_if(in_state(ScreenState::InGame)));
    }
}
