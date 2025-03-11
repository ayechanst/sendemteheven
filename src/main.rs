use bevy::prelude::*;
use plugins::world::WorldPlugin;
use systems::spawn_tile::spawn_tile;

mod plugins;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
        .add_systems(Startup, spawn_tile)
        .run();
}
