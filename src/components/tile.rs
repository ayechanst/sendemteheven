use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {
    pub kind: TileKind,
}

#[derive(Component)]
pub enum TileKind {
    stone,
    mud,
    water,
}
