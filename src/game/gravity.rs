use std::collections::{HashMap, HashSet};

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::game::{
    scenes::scene_in_game::PlayerShip,
    util::{dynamic_grid::*, granular_grid::BiLerp},
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GravitySystems;

pub struct GravityPlugin {
    gravitational_constant: f32,
    granularity_function: fn(Vec2) -> u32,
    grid_side_length: f32,
    grid_grow_limit_factor: f32,
    gravity_type: GravityType,
}

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GravitySetup::new(
            self.gravity_type,
            self.gravitational_constant,
            self.granularity_function,
        ))
        .insert_resource(GravityGrids::new(
            self.grid_side_length,
            self.grid_side_length * self.grid_grow_limit_factor,
        ))
        .add_systems(
            FixedUpdate,
            (update_gravity_grids, apply_gravity).chain().in_set(GravitySystems),
        );
        let world = app.world_mut();
        world.spawn((Observer::new(on_insert_massive_object), Name::new("OnInsertMassiveObject")));
        world.spawn((Observer::new(on_remove_massive_object), Name::new("OnRemoveMassiveObject")));
    }
}

impl Default for GravityPlugin {
    fn default() -> Self {
        Self {
            gravitational_constant: 1.,
            granularity_function: |position: Vec2| {
                let distance_units = (position.length() - 0.5).max(0.);
                (3. * 0.6f32.powf(distance_units)) as u32
            },
            grid_side_length: 100.,
            grid_grow_limit_factor: 3.,
            gravity_type: GravityType::G3D,
        }
    }
}

#[derive(Component, Debug, Default, Reflect)]
pub struct GravitationalMass(pub f32);

#[derive(Component, Debug, Reflect)]
#[require(GravitationalMass)]
pub struct GravitationalAttraction;

#[derive(Component, Debug, Reflect)]
#[require(GravitationalMass)]
pub struct GravitationalPull {
    pub mass_radius: f32, // The mass is modelled as a solid sphere rather than a point source.
}

#[derive(Clone, Copy, Debug, Default, Reflect)]
pub struct GravityValue {
    pub potential:      f32,
    pub field_strength: Vec2,
}

#[derive(Clone, Copy, Debug, Reflect)]
pub enum GravityType {
    G2D,
    G3D,
}

#[derive(Clone, Copy, Debug, Reflect)]
struct MassiveObject {
    mass:     f32,
    radius:   f32,
    position: Vec2,
}

fn minimum_granularity() -> fn(Vec2) -> u32 {
    |_| 0
}
#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct GravitySetup {
    pub gravity_type: GravityType,
    pub gravitational_constant: f32,
    #[reflect(ignore, default = "minimum_granularity")]
    pub granularity_fn: fn(Vec2) -> u32,
    massive_objects: HashMap<Entity, MassiveObject>,
    waiting_to_be_processed: HashSet<Entity>,
}

impl GravitySetup {
    fn new(
        gravity_type: GravityType,
        gravitational_constant: f32,
        granularity_fn: fn(Vec2) -> u32,
    ) -> Self {
        let massive_objects = HashMap::new();
        let waiting_to_be_processed = HashSet::new();
        Self {
            gravity_type,
            gravitational_constant,
            granularity_fn,
            massive_objects,
            waiting_to_be_processed,
        }
    }
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct GravityGrids {
    pub grid_add_radius: f32,
    pub the_grids: DynamicGrid<GravityValue>,
}

impl GravityGrids {
    pub fn new(grid_side_length: f32, grid_add_radius: f32) -> Self {
        Self {
            grid_add_radius,
            the_grids: DynamicGrid::new(grid_side_length),
        }
    }
}

impl BiLerp for GravityValue {
    fn bilerp(
        top_left: &Self,
        top_right: &Self,
        bottom_left: &Self,
        bottom_right: &Self,
        s: Vec2,
    ) -> Self {
        let potential_top = top_left.potential.lerp(top_right.potential, s.x);
        let potential_bottom = bottom_left.potential.lerp(bottom_right.potential, s.x);
        let potential = potential_bottom.lerp(potential_top, s.y);
        let field_top = top_left.field_strength.lerp(top_right.field_strength, s.x);
        let field_bottom = bottom_left.field_strength.lerp(bottom_right.field_strength, s.x);
        let field_strength = field_bottom.lerp(field_top, s.y);
        Self {
            potential,
            field_strength,
        }
    }
}

impl std::ops::Add<GravityValue> for GravityValue {
    type Output = Self;

    fn add(self, rhs: GravityValue) -> Self::Output {
        GravityValue {
            potential:      self.potential + rhs.potential,
            field_strength: self.field_strength + rhs.field_strength,
        }
    }
}

impl std::ops::Sub<GravityValue> for GravityValue {
    type Output = Self;

    fn sub(self, rhs: GravityValue) -> Self::Output {
        GravityValue {
            potential:      self.potential - rhs.potential,
            field_strength: self.field_strength - rhs.field_strength,
        }
    }
}

impl std::ops::AddAssign for GravityValue {
    fn add_assign(&mut self, rhs: Self) {
        self.potential += rhs.potential;
        self.field_strength += rhs.field_strength;
    }
}

pub fn apply_gravity(
    gravity_grids: Res<GravityGrids>,
    mut mass_query: Query<(Forces, &GravitationalMass, &Position), With<GravitationalAttraction>>,
) {
    for (mut forces, mass, position) in mass_query.iter_mut() {
        let mass = mass.0;
        let position = position.truncate();
        let local_gravity = gravity_grids.the_grids.value_at(position);
        let local_force = local_gravity.field_strength * mass;
        forces.apply_force(local_force.extend(0.));
    }
}

pub fn update_gravity_grids(
    mut gravity_setup: ResMut<GravitySetup>,
    mut gravity_grids: ResMut<GravityGrids>,
    player_position: Single<&Position, With<PlayerShip>>,
    massive_objects_query: Query<(&Position, &GravitationalMass, &GravitationalPull)>,
) {
    let unprocessed = gravity_setup.waiting_to_be_processed.clone();
    let player_position = player_position.into_inner().truncate();
    for &entity in &unprocessed {
        if let Ok((position, mass, pull)) = massive_objects_query.get(entity) {
            let new_massive_object = MassiveObject {
                mass:     mass.0,
                position: position.0.truncate(),
                radius:   pull.mass_radius,
            };
            let delta_fn = create_delta_fn(
                gravity_setup.gravity_type,
                gravity_setup.gravitational_constant,
                Some(new_massive_object),
                None,
            );
            gravity_grids.the_grids.update_grid_values(delta_fn);
            gravity_setup.massive_objects.insert(entity, new_massive_object);
            gravity_setup.waiting_to_be_processed.remove(&entity);
        }
    }
    let gravity_fn = create_gravity_fn(
        gravity_setup.gravity_type,
        gravity_setup.gravitational_constant,
        &gravity_setup.massive_objects,
    );
    gravity_grids.the_grids.update_grid_granularities(gravity_setup.granularity_fn, &gravity_fn);
    let grid_add_radius = gravity_grids.grid_add_radius;
    gravity_grids.the_grids.add_grids_within_range(
        player_position,
        grid_add_radius,
        gravity_setup.granularity_fn,
        &gravity_fn,
    );
}

pub fn on_insert_massive_object(
    event: On<Insert, GravitationalPull>,
    mut global_gravity: ResMut<GravitySetup>,
) {
    if global_gravity.waiting_to_be_processed.contains(&event.entity) {
        // Object never got added before new component got inserted, so leave it there
    }
    else {
        global_gravity.waiting_to_be_processed.insert(event.entity);
    }
}

pub fn on_remove_massive_object(
    event: On<Remove, GravitationalPull>,
    mut gravity_setup: ResMut<GravitySetup>,
    mut gravity_grids: ResMut<GravityGrids>,
) {
    let entity = &event.entity;
    if gravity_setup.waiting_to_be_processed.remove(entity) {
        // Object never got added so nothing more to do
    }
    else if let Some(massive_object) = gravity_setup.massive_objects.remove(entity) {
        let gravity_update_fn = create_delta_fn(
            gravity_setup.gravity_type,
            gravity_setup.gravitational_constant,
            None,
            Some(massive_object),
        );
        gravity_grids.the_grids.update_grid_values(gravity_update_fn);
        gravity_setup.massive_objects.remove(entity);
    }
    else {
        panic!("Removed massive object ({entity}) not present in global gravity");
    }
}

fn create_gravity_fn(
    gravity_type: GravityType,
    gravitational_constant: f32,
    masses: &HashMap<Entity, MassiveObject>,
) -> impl Fn(Vec2) -> GravityValue {
    move |position: Vec2| {
        let mut gravity_value = GravityValue::default();
        for massive_object in masses.values() {
            gravity_value +=
                calc_gravity_from(gravity_type, gravitational_constant, position, massive_object);
        }
        gravity_value
    }
}

fn create_delta_fn(
    gravity_type: GravityType,
    gravitational_constant: f32,
    new_massive_object: Option<MassiveObject>,
    old_massive_object: Option<MassiveObject>,
) -> impl Fn(Vec2) -> GravityValue {
    move |local_position: Vec2| {
        let a = match new_massive_object {
            Some(massive_object) => calc_gravity_from(
                gravity_type,
                gravitational_constant,
                local_position,
                &massive_object,
            ),
            None => GravityValue::default(),
        };
        let b = match old_massive_object {
            Some(massive_object) => calc_gravity_from(
                gravity_type,
                gravitational_constant,
                local_position,
                &massive_object,
            ),
            None => GravityValue::default(),
        };
        a - b
    }
}

/// Calculates the gravity at a local position caused by a distant massive object.
fn calc_gravity_from(
    gravity_type: GravityType,
    gravitational_constant: f32,
    local_position: Vec2,
    massive_object: &MassiveObject,
) -> GravityValue {
    let distance = massive_object.position - local_position;
    let scaling = massive_object.mass * gravitational_constant;
    match gravity_type {
        GravityType::G2D => calc_2d_gravity(massive_object.radius, distance, scaling),
        GravityType::G3D => calc_3d_gravity(massive_object.radius, distance, scaling),
    }
}
fn calc_2d_gravity(r: f32, dist: Vec2, scaling: f32) -> GravityValue {
    let d = dist.length();
    let r2inv = 1. / (r * r);
    let mut potential;
    let mut field_strength;
    if d < r {
        potential = r.ln() + 0.5 * d * d * r2inv - 0.5;
        field_strength = dist * r2inv;
    }
    else {
        potential = r.ln();
        field_strength = dist / (d * d);
    }
    potential *= scaling;
    field_strength *= scaling;
    GravityValue {
        potential,
        field_strength,
    }
}

fn calc_3d_gravity(r: f32, dist: Vec2, scaling: f32) -> GravityValue {
    let d = dist.length();
    let r3inv = 1. / (r * r * r);
    let mut potential;
    let mut field_strength;
    if d < r {
        potential = 0.5 * r3inv * (d * d - 3. * r * r);
        field_strength = dist * r3inv;
    }
    else {
        potential = -1. / d;
        field_strength = dist / (d * d * d);
    }
    potential *= scaling;
    field_strength *= scaling;
    GravityValue {
        potential,
        field_strength,
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use linspace::Linspace;

    use super::*;
    use std::time::Duration;

    const TIME_STEP: f32 = 0.010; // 10 ms

    fn build_app() -> App {
        let mut app = App::new();
        app.add_plugins((
            (
                MinimalPlugins,
                bevy::transform::TransformPlugin,
                bevy::asset::AssetPlugin::default(),
                bevy::scene::ScenePlugin,
            ),
            PhysicsPlugins::default(),
            GravityPlugin {
                gravitational_constant: 1.,
                granularity_function: |_| 5,
                grid_side_length: 100.,
                grid_grow_limit_factor: 1.,
                gravity_type: GravityType::G3D,
            },
        ));
        app.finish();
        app.cleanup();
        // Something Avian needs that is not setup by MinimalPlugins
        app.init_resource::<Assets<Mesh>>().add_message::<AssetEvent<Mesh>>();
        app.insert_resource(Gravity::ZERO);
        app
    }

    fn add_player_ship(app: &mut App, position: Vec2) -> Entity {
        app.world_mut().spawn((PlayerShip::default(), Position::new(position.extend(0.)))).id()
    }

    fn add_massive_object(app: &mut App, position: Vec2) -> Entity {
        app.world_mut()
            .spawn((
                Position::new(position.extend(0.)),
                GravitationalMass(1.),
                GravitationalPull { mass_radius: 0.1 },
            ))
            .id()
    }

    fn set_timestep(app: &mut App, time_step: f32) {
        app.world_mut()
            .get_resource_mut::<Time<Fixed>>()
            .unwrap()
            .set_timestep(Duration::from_secs_f32(time_step));
    }

    #[test]
    fn test_gravity_grid_setup() {
        let mut app = build_app();
        set_timestep(&mut app, TIME_STEP);
        let player_ship = add_player_ship(&mut app, Vec2::new(0., 0.));
        {
            let gravity_grids = app.world().get_resource::<GravityGrids>().unwrap();
            assert!(gravity_grids.the_grids.is_empty());
            let gravity_setup = app.world().get_resource::<GravitySetup>().unwrap();
            assert!(gravity_setup.waiting_to_be_processed.is_empty());
            assert!(gravity_setup.massive_objects.is_empty());
        }
        let massive_object = add_massive_object(&mut app, Vec2::new(0., 0.));
        {
            let gravity_grids = app.world().get_resource::<GravityGrids>().unwrap();
            assert!(gravity_grids.the_grids.is_empty());
            let gravity_setup = app.world().get_resource::<GravitySetup>().unwrap();
            assert_eq!(gravity_setup.waiting_to_be_processed.len(), 1);
            assert!(gravity_setup.massive_objects.is_empty());
        }
        app.update();
        std::thread::sleep(Duration::from_secs_f32(TIME_STEP));
        app.update();
        {
            let gravity_grids = app.world().get_resource::<GravityGrids>().unwrap();
            assert_eq!(gravity_grids.the_grids.len(), 9);
            let gravity_setup = app.world().get_resource::<GravitySetup>().unwrap();
            assert!(gravity_setup.waiting_to_be_processed.is_empty());
            assert_eq!(gravity_setup.massive_objects.len(), 1);
            assert_eq!(gravity_grids.the_grids.value_at(Vec2::ZERO).field_strength, Vec2::ZERO,);
            for x in (10f32..100f32).linspace(5) {
                for y in (10f32..100f32).linspace(5) {
                    assert_relative_eq!(
                        gravity_grids.the_grids.value_at(Vec2::new(x, y)).field_strength,
                        Vec2::new(-x, -y) / (x.powi(2) + y.powi(2)).powf(1.5),
                        max_relative = 0.05
                    );
                }
            }
        }
        app.world_mut().get_mut::<Position>(player_ship).unwrap().x = 100.;
        app.world_mut().despawn(massive_object);
        std::thread::sleep(Duration::from_secs_f32(TIME_STEP));
        app.update();
        {
            let gravity_grids = app.world().get_resource::<GravityGrids>().unwrap();
            assert_eq!(gravity_grids.the_grids.len(), 12);
            let gravity_setup = app.world().get_resource::<GravitySetup>().unwrap();
            assert!(gravity_setup.waiting_to_be_processed.is_empty());
            assert!(gravity_setup.massive_objects.is_empty());
            assert_eq!(gravity_grids.the_grids.value_at(Vec2::ZERO).field_strength, Vec2::ZERO,);
            for x in (10f32..100f32).linspace(5) {
                for y in (10f32..100f32).linspace(5) {
                    assert_relative_eq!(
                        gravity_grids.the_grids.value_at(Vec2::new(x, y)).field_strength,
                        Vec2::splat(0.),
                        max_relative = 0.05
                    );
                }
            }
        }
    }

    #[test]
    fn test_gravity_attraction() {
        let mut app = build_app();
        set_timestep(&mut app, TIME_STEP);
        add_player_ship(&mut app, Vec2::new(0., 0.));
        let attractor = add_massive_object(&mut app, Vec2::new(0., 0.));
        let attractee = app
            .world_mut()
            .spawn((
                Position(Vec2::new(-100., 0.).extend(0.)),
                GravitationalMass(1.),
                GravitationalAttraction,
                Mass(1.),
                RigidBody::Dynamic,
            ))
            .id();

        app.update(); // Massive object queued up
        std::thread::sleep(Duration::from_secs_f32(TIME_STEP));
        app.update(); // Gravity ready

        let speed = app.world().get::<LinearVelocity>(attractee).unwrap().0.length();
        let v = TIME_STEP / 100f32.powi(2);
        assert_relative_eq!(speed, v, max_relative = 0.001);

        app.world_mut().despawn(attractor);
        std::thread::sleep(Duration::from_secs_f32(TIME_STEP));
        app.update(); // Gravity ready

        let speed = app.world().get::<LinearVelocity>(attractee).unwrap().0.length();
        assert_relative_eq!(speed, v, max_relative = 0.001);
    }
}
