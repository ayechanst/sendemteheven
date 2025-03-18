use bevy::prelude::*;

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
}

// #[derive(Component)]
// struct Speed(f32);

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("assassino.png");
    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: 100.0 },
    ));
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

// fn player_movement(
//     keys: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
//     mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
//     cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
// ) {
//     for (mut player_transform, player_speed) in player_q.iter_mut() {
//         let cam = match cam_q.get_single() {
//             Ok(c) => c,
//             Err(e) => Err(format!("Error getting camera: {}", e)).unwrap(),
//         };
//         let mut direction = Vec3::ZERO;
//         if keys.pressed(KeyCode::KeyW) {
//             direction += *cam.forward();
//         }
//         if keys.pressed(KeyCode::KeyS) {
//             direction += *cam.back();
//         }
//         if keys.pressed(KeyCode::KeyA) {
//             direction += *cam.left();
//         }
//         if keys.pressed(KeyCode::KeyD) {
//             direction += *cam.right();
//         }
//         direction.y = 0.0;
//         let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
//         player_transform.translation += movement;
//     }
// }

// fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let texture = asset_server.load("assassino.png");
//     println!("Loaded texture: {:?}", texture);
//     let player = (
//         SpriteBundle {
//             // texture: asset_server.load("assassino.png"),
//             texture: texture,
//             transform: Transform::from_xyz(0.0, 0.0, 0.0),
//             ..default()
//         },
//         Speed(30.0),
//         Player,
//     );
//     commands.spawn(player);
// }
