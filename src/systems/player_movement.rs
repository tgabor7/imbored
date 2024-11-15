use std::ops::Sub;

use bevy::{prelude::*, window::PrimaryWindow};
use crate::components::{acceleration::Acceleration, main_camera::MainCamera, physics::Physics, player::Player, projectile::Projectile, velocity::Velocity};

pub fn player_movement(mut commands: Commands, 
                       mouse_buttons: Res<ButtonInput<MouseButton>>, 
                       keyboard_input: Res<ButtonInput<KeyCode>>, 
                       q_windows: Query<&Window, With<PrimaryWindow>>,
                       q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
                       mut query: Query<(&mut Velocity, &mut Transform), With<Player>>, time: Res<Time>) {

    let delta = time.delta_seconds();

    for mut player in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            player.0.y += 1000.0 * delta;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            player.0.y -= 1000.0 * delta;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            player.0.x -= 1000.0 * delta;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            player.0.x += 1000.0 * delta;
        }

        let (camera, camera_transform) = q_camera.single();

        let mut cursor_position = Vec2::new(0.0, 0.0);

        // There is only one primary window, so we can similarly get it from the query:
        let window = q_windows.single();

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            cursor_position = world_position;
        }
        
        let player_position = player.1.translation;

        let direction = cursor_position.sub(player_position.truncate()).normalize();

        let projectile_initial_velocity = Velocity { x: direction.x * 1000.0, y: direction.y * 1000.0 };

        if mouse_buttons.pressed(MouseButton::Left) {

            commands.spawn((SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.55, 0.05),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(player.1.translation.x, player.1.translation.y, 0.)),
                ..default()
            }, Physics { friction: 1. }, Projectile { life: 1. }, projectile_initial_velocity, Acceleration { x: 10., y: 0. }));

        }
    }

}
