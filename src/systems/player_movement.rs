use bevy::prelude::*;
use crate::components::player::Player;

pub fn player_movement(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {

    let delta = time.delta_seconds();

    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.y += 100.0 * delta;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 100.0 * delta;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 100.0 * delta;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x += 100.0 * delta;
        }
    }

}
