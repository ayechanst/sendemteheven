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
        // .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(TiledPhysicsPlugin::<TiledPhysicsRapierBackend>::default())
        // .add_plugins(PhysicsPlugins::default().with_length_unit(100.0))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<TiledMap> = asset_server.load("map-v3.tmx");

    commands.spawn((
        TiledMapHandle(handle),
        RigidBody::Fixed,
        // Collider::ball(50.5),
        TilemapRenderSettings {
            render_chunk_size: UVec2::new(64, 1),
            y_sort: true,
        },
        TiledMapAnchor::Center,
        // TiledPhysicsSettings::<TiledPhysicsRapierBackend> {

        // },
    ));

    commands.spawn((
        Camera2d,
        Transform {
            translation: Vec3::new(0.0, 0.0, 100.0), // Center the camera (at origin in this case)
            scale: Vec3::splat(0.5),                 // Zoom in: < number
            ..Default::default()
        },
    ));
}
