use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tiled::TiledMapPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use plugins::player::Player;

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
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(TilemapPlugin)
        .add_plugins(TiledMapPlugin::default())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    let texture = asset_server.load("assassino.png");
    let map_handle: Handle<TiledMap> = asset_server.load("test-tilemap.tmx");

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);
    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: 100.0 },
    ));
    commands.spawn(TiledMapHandle(map_handle));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= movement_amount;
        }
    }
}
