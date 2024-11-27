use bevy::{
    ecs::system::Query,
    prelude::*,
    prelude::{Commands, Entity, Transform, With, Without},
};
use rand::Rng;

use crate::{
    components::{
        acceleration::Acceleration,
        enemy::Enemy,
        player::{self, Player},
        projectile::Projectile,
        velocity::Velocity,
    },
    utils::collision::{
        check_collision, check_collision_with_velocity, check_swept_aabb_collision,
    },
};

pub fn enemy_collision(
    mut enemies: Query<(&Transform, &mut Velocity, &Enemy), (With<Enemy>, Without<Projectile>)>,
    mut projectiles: Query<(&mut Transform, &mut Projectile, &mut Velocity)>,
    time: Res<Time>,
) {
    for (enemy_transform, mut enemy_velocity, _) in enemies.iter_mut() {
        for (mut projectile_transform, mut projectile, mut velocity) in projectiles.iter_mut() {
            let delta = time.delta_seconds();

            let (collision, entry_time, exit_time, normalx, normaly) = check_swept_aabb_collision(
                &projectile_transform,
                enemy_transform,
                &Velocity {
                    x: velocity.x * delta * 2.,
                    y: velocity.y * delta * 2.,
                },
                &enemy_velocity,
                delta,
            );

            if collision {
                println!("Enemy hit!");

                projectile_transform.translation.x =
                    projectile_transform.translation.x + velocity.x * delta * entry_time;
                projectile_transform.translation.y =
                    projectile_transform.translation.y + velocity.y * delta * entry_time;

                let remaining_time = 1.0 - entry_time;
                velocity.x = velocity.x * exit_time * delta;
                velocity.y = velocity.y * exit_time * delta;

                if normalx.abs() > 0.0001 {
                    velocity.x = -velocity.x;
                }
                if normaly.abs() > 0.0001 {
                    velocity.y = -velocity.y;
                }
            }
        }
    }
}

pub fn player_collision(
    mut enemies: Query<(&Transform, &mut Velocity, &Enemy), (With<Enemy>, Without<Player>)>,
    mut player: Query<(&mut Transform, &mut Player, &mut Velocity)>,
    time: Res<Time>,
) {
    let mut player = player.single_mut();
    let delta = time.delta_seconds();

    for (enemy_transform, enemy_velocity, _) in enemies.iter_mut() {
        let (collision, entry_time, exit_time, normalx, normaly) = check_swept_aabb_collision(
            &player.0,
            enemy_transform,
            &Velocity {
                x: player.2.x * delta,
                y: player.2.y * delta,
            },
            &enemy_velocity,
            delta,
        );

        if collision {
            println!("entry_time: {}", entry_time);
            println!("delta: {}", delta);

            player.0.translation.x = player.0.translation.x + player.2.x * delta * entry_time;
            player.0.translation.y = player.0.translation.y + player.2.y * delta * entry_time;

            let remaining_time = 1.0 - entry_time;
            // player.2.x = player.2.x * remaining_time;
            // player.2.y = player.2.y * remaining_time;

            if normalx.abs() > 0.0001 {
                player.2.x = -player.2.x;
            }
            if normaly.abs() > 0.0001 {
                player.2.y = -player.2.y;
            }
        }
    }
}

pub fn enemy_targeting(
    mut enemies: Query<(&Enemy, &Transform, &mut Velocity)>,
    player: Query<(&Player, &Transform)>,
) {
    let player = player.single();

    let player_position = player.1.translation;

    for mut enemy in enemies.iter_mut() {
        if enemy.0.target == false {
            continue;
        }
        let direction = player_position - enemy.1.translation;

        let direction = direction.normalize();

        enemy.2.x = direction.x * 100.0;
        enemy.2.y = direction.y * 100.0;
    }
}
