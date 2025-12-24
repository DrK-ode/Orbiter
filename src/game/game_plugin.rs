use bevy::app::Plugin;
use bevy::prelude::*;

use super::assets::AssetsPlugin;
use super::physics::PhysicsPlugin;
use super::view::ViewPlugin;

use super::game_states::*;
use super::game_systems::*;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<ScreenState>()
        .add_sub_state::<LoadingState>()
        .add_sub_state::<InGameState>()
        .add_plugins(AssetsPlugin)
        .add_plugins(ViewPlugin)
        .add_plugins(PhysicsPlugin)
        .configure_sets(Update, GameSystemSet::Physics.run_if(in_state(ScreenState::InGame)))
        .add_systems(OnEnter(ScreenState::Splash), spawn_splash_screen)
        .add_systems(OnExit(ScreenState::Splash), teardown_splash_screen)
        .add_systems(OnEnter(ScreenState::Loading), spawn_loading_screen)
        .add_systems(OnExit(ScreenState::Loading), teardown_loading_screen)
        .add_systems(OnEnter(ScreenState::Title), spawn_title_screen)
        .add_systems(OnExit(ScreenState::Title), teardown_title_screen)
        .add_systems(OnEnter(ScreenState::InGame), spawn_in_game_screen)
        .add_systems(OnExit(ScreenState::InGame), teardown_in_game_screen)
        .add_systems(OnEnter(ScreenState::GameOver), spawn_game_over_screen)
        .add_systems(OnExit(ScreenState::GameOver), teardown_game_over_screen);
        #[cfg(debug_assertions)]
        app.add_systems(Update, (log_state_change_request.run_if(resource_changed::<NextState<ScreenState>>),
        log_state_change.run_if(state_changed::<ScreenState>)));
        app.add_systems(
            Update,
            (
                transition_from_splash_to_title.run_if(in_state(ScreenState::Splash)),
                transition_from_title_to_loading.run_if(in_state(ScreenState::Title)),
                transition_from_loading_to_in_game.run_if(in_state(ScreenState::Loading)),
                transition_from_in_game_to_game_over.run_if(in_state(ScreenState::InGame)),
                transition_from_game_over_to_title.run_if(in_state(ScreenState::GameOver)),
            )
        );
    }
}
