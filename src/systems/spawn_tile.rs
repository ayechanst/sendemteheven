use bevy::prelude::*;
// mod resources;

pub fn spawn_tile(
    mut commands: Commands,
    // mut tile_map: ResMut<TileMap>,
    asset_server: ResMut<AssetServer>,
) {
    let texture_handle = asset_server.load("assets/block.png");
    commands.spawn(SpriteBundle {
        texture: texture_handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
