use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tiled::TiledMapPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use plugins::player::PlayerPlugin;

mod plugins;
mod systems;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Kill fucking everyone".into(),
                        resolution: (640.0, 480.0).into(),
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(TilemapPlugin)
        .add_plugins(TiledMapPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_loaded_map)
        .run();
}

#[derive(Component)]
struct PendingMap;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    let map_handle: Handle<TiledMap> = asset_server.load("test-tilemap.tmx");
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 3500.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    };

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);
    println!("map about to spawn");
    commands.spawn((
        TiledMapHandle(map_handle),
        PendingMap,
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
    println!("the map spawned i guess?");
    commands.spawn(light);
}

fn spawn_loaded_map(
    mut commands: Commands,
    tiled_maps: Res<Assets<TiledMap>>,
    query: Query<(Entity, &Handle<TiledMap>), With<PendingMap>>,
) {
    for (entity, map_handle) in query.iter() {
        if tiled_maps.contains(map_handle) {
            commands.entity(entity).remove::<PendingMap>(); // Remove the tag
            println!("Map successfully spawned!");
        }
    }
}

// fn check_map_ready(tiled_maps: Res<Assets<TiledMap>>, query: Query<(Entity, &Handle<TiledMap>)>) {
//     for (entity, map_handle) in query.iter() {
//         if tiled_maps.contains(map_handle) {
//             println!("Map loaded successfully!");
//         }
//     }
// }
