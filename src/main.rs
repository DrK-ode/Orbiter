use bevy::prelude::*;
use orbiter::{
    assets::AssetsPlugin, dev_tools::DevToolsPlugin, physics::PhysicsPlugin, game_world::GameWorldPlugin, view::ViewPlugin
};

fn main() -> AppExit {
    let mut app = App::new();
    app.insert_resource(Time::<Fixed>::from_hz(30.));
    app.add_plugins(DefaultPlugins)
        .add_plugins(AssetsPlugin)
        .add_plugins(ViewPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(GameWorldPlugin);
    #[cfg(debug_assertions)]
    app.add_plugins(DevToolsPlugin);
    app.run()
}
