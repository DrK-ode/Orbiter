pub mod force;
pub mod input;
pub mod motion;
mod physics_plugin;
pub mod physics_traits;
pub use physics_plugin::PhysicsPlugin;

pub mod prelude {
    pub use super::{
        force::force_components::*, motion::motion_components::*, physics_traits::*, PhysicsPlugin,
    };
}
