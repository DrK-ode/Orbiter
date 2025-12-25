use bevy::prelude::*;

use crate::game::{
    assets::asset_resources::*, input::input_components::*, physics::prelude::*, scenes::GameScene,
};

#[derive(Debug, Component, Reflect)]
pub struct PlayerShip {
    pub thrust: f32,
    pub reorient_mode: ReorientMode,
}

impl Default for PlayerShip {
    fn default() -> Self {
        Self {
            thrust: 1.,
            reorient_mode: Default::default(),
        }
    }
}

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect)]
#[source(GameScene = GameScene::InGame)]
pub enum InGameState {
    #[default]
    Starting,
    Playing,
    Quit,
    Paused,
}

pub fn plugin_scene_in_game(app: &mut App) {
    app.add_sub_state::<InGameState>()
        .add_systems(OnEnter(GameScene::InGame), spawn_in_game_screen)
        .add_systems(OnExit(GameScene::InGame), teardown_in_game_screen);
    app.add_systems(Update, transition_from_in_game.run_if(in_state(GameScene::InGame)));
}

pub fn spawn_in_game_screen(
    mut commands: Commands,
    meshes: Res<Meshes>,
    materials: Res<Materials>,
) {
    commands.spawn(player_ship_bundle(&meshes, &materials));
    commands.spawn(player_reticle_bundle(&meshes, &materials));
}
pub fn transition_from_in_game(
    mut next_state: ResMut<NextState<GameScene>>,
    sub_state: Res<State<InGameState>>,
) {
    if let InGameState::Quit = sub_state.get() {
        next_state.set(GameScene::GameOver);
    };
}
pub fn teardown_in_game_screen() {}

fn player_ship_bundle(meshes: &Res<Meshes>, materials: &Res<Materials>) -> impl Bundle {
    (
        Name::new("PlayerShip"),
        DespawnOnExit(GameScene::InGame),
        PlayerShip {
            thrust: 1.,
            reorient_mode: Default::default(),
        },
        Mesh3d(meshes.get(MeshType::PlayerShip)),
        MeshMaterial3d(materials.get(MaterialType::PlayerShip)),
        Transform::from_xyz(0., 0., 0.),
        Position::new((0., 0.).into()),
        Velocity::default(),
        ValueLimit::<Velocity>::new(1.),
        Acceleration::default(),
        ForceAndInertia::new(1.),
        Direction::new(Dir2::Y),
        TargetDirection::new(Dir2::Y),
        AngularVelocity::default(),
        ValueLimit::<AngularVelocity>::new(2.),
    )
}

fn player_reticle_bundle(meshes: &Res<Meshes>, materials: &Res<Materials>) -> impl Bundle {
    (
        Name::new("PlayerReticle"),
        DespawnOnExit(GameScene::InGame),
        PlayerReticle,
        Mesh3d(meshes.get(MeshType::Crosshair)),
        MeshMaterial3d(materials.get(MaterialType::Crosshair)),
        Transform::from_xyz(0., 0., 0.1),
        Position::new((0., 0.).into()),
        Velocity::default(),
    )
}
