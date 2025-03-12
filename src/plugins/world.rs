use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera));
    }
}

// fn spawn_light(mut commands: Commands) {
//     let light = PointLightBundle {
//         point_light: PointLight {
//             intensity: 3500.0,
//             ..default()
//         },
//         transform: Transform::from_xyz(0.0, 0.5, 0.0),
//         ..default()
//     };
//     commands.spawn(light);
// }

// fn spawn_camera(mut commands: Commands) {
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//         // transform: Transform::from_xyz(0.0, 2.0, 5.0),
//         ..default()
//     });
// }

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            // clear_color: ClearColorConfig::Custom(Color::GREEN),
        },
        ..default()
    });
}
