use crate::components::{animation_config::AnimationConfig, animation_timer::AnimationTimer};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, animate_player)
            .add_systems(Update, player_movement);
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
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn sprite_row(&self) -> usize {
        match self {
            Direction::Up => 2,
            Direction::Down => 0,
            Direction::Right => 0,
            Direction::Left => 1,
        }
    }
}

const COLUMNS: usize = 5;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("walking-assassino.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 5, 3, None, None);
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
        Collider::capsule_y(5.0, 7.0),
        Player {
            speed: 50.0,
            is_moving: false,
            direction: Direction::Down,
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
        let row_offset = player.direction.sprite_row() * COLUMNS;
        if player.is_moving {
            if config.frame_timer.finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = if atlas.index >= row_offset + config.last_sprite_index {
                        row_offset + config.first_sprite_index
                    } else {
                        atlas.index + 1
                    };
                }
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        } else {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = row_offset + config.first_sprite_index;
            }
        }
    }
}

fn player_movement(
    mut query: Query<(&mut Transform, &mut Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        let mut direction_vec = Vec2::ZERO;
        if input.pressed(KeyCode::KeyW) {
            direction_vec.y += 1.0;
            player.direction = Direction::Up;
        }
        if input.pressed(KeyCode::KeyS) {
            direction_vec.y -= 1.0;
            player.direction = Direction::Down;
        }
        if input.pressed(KeyCode::KeyD) {
            direction_vec.x += 1.0;
            player.direction = Direction::Right;
        }
        if input.pressed(KeyCode::KeyA) {
            direction_vec.x -= 1.0;
            player.direction = Direction::Left;
        }
        if direction_vec != Vec2::ZERO {
            player.is_moving = true;
            direction_vec = direction_vec.normalize();
            let movement_amount = player.speed * time.delta_secs();
            transform.translation += direction_vec.extend(0.0) * movement_amount;
        } else {
            player.is_moving = false;
        }
    }
}
