use bevy::{
    ecs::system::Query,
    prelude::*,
    prelude::{Commands, Entity, Transform, With, Without},
};
use bevy_rapier2d::{
    plugin::RapierContext,
    prelude::{CollisionEvent, Velocity},
};
use rand::Rng;

use crate::{
    components::{
        acceleration::Acceleration,
        enemy::Enemy,
        player::{self, Player},
        projectile::Projectile,
    },
    utils::collision::{
        check_collision, check_collision_with_velocity, check_swept_aabb_collision,
    },
};

pub fn enemy_collision(
    mut enemies: Query<(Entity, &mut Enemy, &Transform)>,
    projectiles: Query<(Entity, &Projectile, &Transform, &Velocity)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(collider1, collider2, flag) => {
                if let Ok((entity, mut enemy, transform)) = enemies.get_mut(*collider1) {
                    if let Ok((projectile_entity, projectile, projectile_transform, velocity)) =
                        projectiles.get(*collider2)
                    {
                        enemy.health = enemy.health - projectile.damage;
                        println!("Enemy hit by projectile");
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn enemy_despawn_system(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Enemy, &Transform)>,
) {
    for (entity, enemy, transform) in enemies.iter_mut() {
        if enemy.health <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
