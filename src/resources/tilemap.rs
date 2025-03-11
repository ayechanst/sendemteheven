use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct TileMap {
    pub tiles: Hashmap<(i32, i32), Entity>,
}
