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
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let map_handle: Handle<TiledMap> = asset_server.load("test2.tmx");
    commands.spawn((
        TiledMapHandle(asset_server.load("test2.tmx")),
        TiledMapAnchor::Center,
    ));
    commands.spawn(Camera2d);
}
