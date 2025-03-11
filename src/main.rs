use bevy::prelude::*;
use plugins::world::WorldPlugin;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
        .run();
}
