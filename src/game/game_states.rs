use bevy::prelude::*;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect)]
pub enum ScreenState {
    #[default]
    Splash,
    Title,
    Loading,
    InGame,
    GameOver,
}

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect)]
#[source(ScreenState = ScreenState::Loading)]
pub enum LoadingState{
    #[default]
    StillLoading,
    DoneLoading
}

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect)]
#[source(ScreenState = ScreenState::InGame)]
pub enum InGameState{
    #[default]
    Starting,
    Playing,
    Quit,
    Paused,
}