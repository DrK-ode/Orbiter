use bevy::prelude::*;
use orbiter::{
    dev_tools::DevToolsPlugin, game::GamePlugin
};

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin);
    #[cfg(debug_assertions)]
    app.add_plugins(DevToolsPlugin);
    app.run()
}
