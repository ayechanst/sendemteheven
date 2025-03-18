use bevy::prelude::*;
use plugins::player::PlayerPlugin;
mod plugins;
fn main() {
    println!("hello new world");
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
