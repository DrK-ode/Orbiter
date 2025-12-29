use avian3d::prelude::*;
use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::css::PINK,
    prelude::*,
    scene::SceneInstanceReady,
};

use crate::game::{
    assets::asset_resources::*,
    input::input_components::*,
    scenes::GameScene,
    view::CAMERA_2D_FOREGROUND_LAYER,
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

#[derive(Debug, Component)]
pub struct PlayerReticle;

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
    ship_assets: Res<PlayerAssets>,
    space_assets: Res<SpaceAssets>,
) {
    commands.spawn((
        Name::new("Mars"),
        DespawnOnExit(GameScene::InGame),
        SceneRoot(space_assets.mars_scene.clone()),
        Transform::from_xyz(10., 0., 0.).with_scale(Vec3::new(0.01, 0.01, 0.01)),
        Collider::sphere(400.),
        Mass(100.),
        NoAutoMass,
        AngularInertia::new(Vec3::new(1., 1., 1.)),
        NoAutoAngularInertia,
        CenterOfMass(Vec3::ZERO),
        AngularVelocity(Vec3::new(0.5, 0.5, 0.5)),
        RigidBody::Kinematic,
    ));
    commands
        .spawn((
            Name::new("PlayerShip"),
            DespawnOnExit(GameScene::InGame),
            PlayerShip {
                thrust: 1.,
                reorient_mode: Default::default(),
            },
            DirectionTarget(Dir2::Y),
            SceneRoot(ship_assets.ship_scene.clone()),
            (
                RigidBody::Dynamic,
                LockedAxes::new().lock_rotation_x().lock_rotation_y().lock_translation_z(),
                NoAutoMass,
                NoAutoAngularInertia,
                NoAutoCenterOfMass,
                CollisionEventsEnabled,
                ActiveCollisionHooks::all(),
                MaxLinearSpeed(1.),
                MaxAngularSpeed(2.),
                Transform::from_xyz(0., 0., 0.),
                Mass(1.),
                CenterOfMass(Vec3::ZERO),
                AngularInertia::new(Vec3::new(1., 1., 1.)),
            ),
        ))
        .observe(add_collider_from_meshes);
    commands.spawn((
        Name::new("PlayerReticle"),
        DespawnOnExit(GameScene::InGame),
        PlayerReticle,
        LockedAxes::ROTATION_LOCKED,
        RigidBody::Kinematic,
        Sprite {
            image: ship_assets.reticle_image.clone(),
            color: PINK.into(),
            custom_size: Some(Vec2::new(50., 50.)),
            ..Default::default()
        },
        RenderLayers::layer(CAMERA_2D_FOREGROUND_LAYER),
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
            commands.entity(entity).insert(CollisionEventsEnabled);
        }
    }
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
