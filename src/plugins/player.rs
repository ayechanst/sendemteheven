use bevy::prelude::*;

// pub struct PlayerPlugin;

// impl Plugin for PlayerPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, spawn_player)
//         // .add_systems(Update, player_movement);
//     }
// }

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

// #[derive(Component)]
// struct Speed(f32);

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
