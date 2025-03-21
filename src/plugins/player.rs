use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, animate_player)
            .add_systems(Update, character_movement);
        // .add_systems(Update, update_player_direction)
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub is_moving: bool,
    pub direction: Direction,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    DownRight,
    DownLeft,
    UpRight,
    UpLeft,
}

impl Direction {
    fn sprite_row(&self) -> usize {
        match self {
            Direction::DownRight => 0,
            Direction::DownLeft => 1,
            Direction::UpRight => 2,
            Direction::UpLeft => 3,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

const COLUMNS: usize = 1;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("walking-assassino.png");
    // let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), COLUMNS as u32, 1, None, None);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 5, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_config_1 = AnimationConfig::new(0, 4, 4);

    commands.spawn((
        Sprite {
            image: texture_handle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config_1.first_sprite_index,
            }),
            ..default()
        },
        // RigidBody::Dynamic,
        Collider::capsule_y(15.0, 30.0),
        Player {
            speed: 50.0,
            is_moving: false,
            direction: Direction::DownRight,
        },
        animation_config_1,
        AnimationTimer(Timer::new(
            Duration::from_secs_f32(1.0),
            TimerMode::Repeating,
        )),
    ));
}

fn animate_player(mut query: Query<(&mut AnimationConfig, &Player, &mut Sprite)>, time: Res<Time>) {
    for (mut config, player, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());
        if player.is_moving {
            if config.frame_timer.finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    println!("Atlas found! Current index: {}", atlas.index);
                    atlas.index = if atlas.index >= config.last_sprite_index {
                        config.first_sprite_index
                    } else {
                        atlas.index + 1
                    };
                }
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        } else {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = config.first_sprite_index;
            }
        }
    }
}

fn character_movement(
    mut query: Query<(&mut Transform, &mut Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        let mut direction_vec = Vec2::ZERO;
        if input.pressed(KeyCode::KeyW) {
            direction_vec.y += 1.0;
            player.direction = Direction::UpRight;
        }
        if input.pressed(KeyCode::KeyS) {
            direction_vec.y -= 1.0;
            player.direction = Direction::DownRight;
        }
        if input.pressed(KeyCode::KeyD) {
            direction_vec.x += 1.0;
            player.direction = Direction::DownLeft;
        }
        if input.pressed(KeyCode::KeyA) {
            direction_vec.x -= 1.0;
            player.direction = Direction::DownLeft;
        }
        if direction_vec != Vec2::ZERO {
            player.is_moving = true;
            direction_vec = direction_vec.normalize();
            let movement_amount = player.speed * time.delta_secs();
            transform.translation += direction_vec.extend(0.0) * movement_amount;
            // println!("player is moving");
        } else {
            player.is_moving = false;
            // println!("player is NOT moving");
        }
        // println!("Player Direction: {:?}", player.direction);
    }
}

// fn update_player_direction(mut query: Query<&mut Player>, input: Res<ButtonInput<KeyCode>>) {
//     for mut player in query.iter_mut() {
//         let up = input.pressed(KeyCode::KeyW);
//         let down = input.pressed(KeyCode::KeyS);
//         let right = input.pressed(KeyCode::KeyD);
//         let left = input.pressed(KeyCode::KeyA);

//         if up && right {
//             player.direction = Direction::UpRight;
//         } else if left && up {
//             player.direction = Direction::UpLeft;
//         } else if down && right {
//             player.direction = Direction::DownRight;
//         } else if down && left {
//             player.direction = Direction::DownLeft;
//         }
//         println!("Player Direction: {:?}", player.direction);
//     }
// }
