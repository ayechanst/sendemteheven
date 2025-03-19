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
    let tile_width = 32.0;
    let tile_height = 16.0;
    let map_handle: Handle<TiledMap> = asset_server.load("right-up.tmx");

    // commands.spawn((TiledMapHandle(map_handle), TiledMapAnchor::Center));
    commands.spawn((
        // Tuple of bundles, such as TiledMapLayer and Visibility
        // The tuple IS the entity
        TiledMapHandle(map_handle.clone()),
        TiledMapLayerZOffset(1.0),
        TilemapRenderSettings {
            render_chunk_size: UVec2::new(64, 1),
            y_sort: true,
        },
        // Visibility::Visible,
        TiledMapAnchor::Center,
    ));
    commands.spawn(Camera2d);
}
