use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use plugins::player::PlayerPlugin;
mod components;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_plugins(TiledMapPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TiledMapHandle(asset_server.load("right-up.tmx")),
        RigidBody::Fixed,
        TilemapRenderSettings {
            render_chunk_size: UVec2::new(64, 1),
            y_sort: true,
        },
        TiledMapAnchor::Center,
    ));
    // commands.spawn(Camera2d);

    commands.spawn((
        Camera2d,
        Camera {
            hdr: true, // HDR is required for the bloom effect
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 100.0), // Center the camera (at origin in this case)
            scale: Vec3::splat(0.75),                // Zoom in 50% (reduce the area covered)
            ..Default::default()
        },
    ));
}
