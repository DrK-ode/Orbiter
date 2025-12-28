use avian3d::prelude::*;
use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::game::{assets::asset_resources::*, input::input_components::*, scenes::CurrentScene};

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

#[derive(Debug, Component)]
pub struct PlayerReticle;

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect)]
#[source(CurrentScene = CurrentScene::InGame)]
pub enum InGameState {
    #[default]
    Starting,
    Playing,
    Quit,
    Paused,
}

pub fn plugin_scene_in_game(app: &mut App) {
    app.add_sub_state::<InGameState>()
        .add_systems(OnEnter(CurrentScene::InGame), spawn_in_game_screen)
        .add_systems(OnExit(CurrentScene::InGame), teardown_in_game_screen);
    app.add_systems(Update, transition_from_in_game.run_if(in_state(CurrentScene::InGame)));
}

pub fn spawn_in_game_screen(mut commands: Commands, ship_assets: Res<PlayerAssets>) {
    commands
        .spawn((
            Name::new("PlayerShip"),
            DespawnOnExit(CurrentScene::InGame),
            PlayerShip {
                thrust: 1.,
                reorient_mode: Default::default(),
            },
            DirectionTarget(Dir2::Y),
            SceneRoot(ship_assets.ship_scene.clone()),
            (
                Transform::from_xyz(0., 0., 0.),
                LockedAxes::new().lock_rotation_x().lock_rotation_y().lock_translation_z(),
                MaxLinearSpeed(1.),
                MaxAngularSpeed(2.),
                Mass(1.),
                NoAutoMass,
                NoAutoCenterOfMass,
                AngularInertia::new(Vec3::new(1., 1., 1.)),
                NoAutoAngularInertia,
                CenterOfMass(Vec3::ZERO),
                RigidBody::Dynamic,
            ),
        ))
        .observe(add_collider_from_meshes);
    commands.spawn((
        Name::new("PlayerReticle"),
        DespawnOnExit(CurrentScene::InGame),
        PlayerReticle,
        Transform::from_xyz(0., 0., 0.1),
        LockedAxes::ROTATION_LOCKED,
        RigidBody::Kinematic,
        Mesh3d(ship_assets.reticle_mesh.clone()),
        MeshMaterial3d(ship_assets.reticle_material.clone()),
    ));
}

fn add_collider_from_meshes(
    event: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    meshes: Query<&Mesh3d>,
    colliders: Query<&Collider>,
) {
    for entity in children.iter_descendants(event.entity) {
        if let Ok(_) = meshes.get(entity)
            && let Err(_) = colliders.get(entity)
        {
            commands.entity(entity).insert(ColliderConstructor::TrimeshFromMesh);
        }
    }
}

pub fn transition_from_in_game(
    mut next_state: ResMut<NextState<CurrentScene>>,
    sub_state: Res<State<InGameState>>,
) {
    if let InGameState::Quit = sub_state.get() {
        next_state.set(CurrentScene::GameOver);
    };
}
pub fn teardown_in_game_screen() {}
