use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use plugins::player::PlayerPlugin;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_plugins(TiledMapPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("test2.tmx");

    // commands.spawn((TiledMapHandle(map_handle), TiledMapAnchor::Center));
    for layer_index in 0..3 {
        commands.spawn((
            TiledMapHandle(map_handle.clone()),
            TiledMapLayer,
            Transform::from_xyz(0.0, 0.0, layer_index as f32),
            Visibility::Visible,
            TiledMapAnchor::Center,
        ));
    }
    commands.spawn(Camera2d);
}
