use bevy::{ecs::{query::{With, Without}, system::{Query, Res}}, time::Time, transform::components::Transform};

use crate::components::{main_camera::MainCamera, player::Player};

pub fn camera_system(players: Query<(&Player, &Transform)>,
mut q_camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
time: Res<Time>) {
    
    let delta = time.delta_seconds();

    // asssume ther's only one player
    for (player, player_transform) in &players {

        let pos = player_transform.translation;

        for mut transform in &mut q_camera {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}
