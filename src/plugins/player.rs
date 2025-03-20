use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, character_movement);
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub is_moving: bool,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let frame_textures: Vec<Handle<Image>> = (1..=11)
        .map(|i| asset_server.load(format!("animated-assassino-export{}.png", i)))
        .collect();
    // let texture_atlas = TextureAtlasLayout::add_texture(&mut self, rect)

    let texture = asset_server.load("assassino.png");
    commands.spawn((
        Sprite {
            image: texture,
            ..default()
        },
        // RigidBody::Dynamic,
        Collider::capsule_y(15.0, 30.0),
        Player {
            speed: 100.0,
            is_moving: false,
        },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_secs();
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
