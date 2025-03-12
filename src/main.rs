use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use plugins::{player::PlayerPlugin, world::WorldPlugin};
use systems::spawn_tile::spawn_tile;

mod plugins;
mod systems;

// pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(WorldInspectorPlugin::new())
        // .add_plugins(WorldPlugin)
        // .add_plugins(PlayerPlugin)
        // .add_systems(Startup, spawn_tile)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let texture = asset_server.load("assassino.png");
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        ..default()
    });
}
