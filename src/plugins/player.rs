use bevy::{input::mouse::MouseMotion, prelude::*};
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
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("assassino.png");
    commands.spawn((
        Sprite {
            image: texture,
            ..default()
        },
        RigidBody::Dynamic,
        Collider::capsule_y(15.0, 30.0),
        Player { speed: 100.0 },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    for mouse_event in mouse_motion_events.read() {
        let mouse_delta = mouse_event.delta;
        for (mut transform, player) in &mut characters {
            let player_pos = transform.translation.truncate();
            let direction = mouse_delta - player_pos;
            let distance = direction.length();
            let stop_distance = 50.0;
            if distance < stop_distance {
                continue;
            }
            let speed_factor = distance / 100.0;
            let movement_amount = player.speed * speed_factor * time.delta_secs();
            let direction_normalized = direction.normalize();
            transform.translation.x += direction_normalized.x * movement_amount;
            transform.translation.y += direction_normalized.y * movement_amount;
        }
    }
}
// fn character_movement(
//     mut characters: Query<(&mut Transform, &Player)>,
//     input: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
// ) {
//     for (mut transform, player) in &mut characters {
//         let movement_amount = player.speed * time.delta_secs();
//         if input.pressed(KeyCode::KeyW) {
//             transform.translation.y += movement_amount;
//         }
//         if input.pressed(KeyCode::KeyS) {
//             transform.translation.y -= movement_amount;
//         }
//         if input.pressed(KeyCode::KeyD) {
//             transform.translation.x += movement_amount;
//         }
//         if input.pressed(KeyCode::KeyA) {
//             transform.translation.x -= movement_amount;
//         }
//     }
// }
