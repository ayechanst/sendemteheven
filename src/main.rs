use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use plugins::world::WorldPlugin;
use systems::spawn_tile::spawn_tile;

mod plugins;
mod systems;

// pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        // .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(WorldPlugin)
        .add_systems(Startup, spawn_tile)
        .run();
}
