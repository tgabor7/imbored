use bevy::{ecs::system::Query, prelude::{Commands, Entity, Transform, With, Without}};

use crate::components::{acceleration::Acceleration, enemy::Enemy, player::{self, Player}, projectile::Projectile, velocity::Velocity};

pub fn enemy_collision(
  mut enemies: Query<(&Transform, &mut Velocity, &Enemy), (With<Enemy>, Without<Projectile>)>, 
  mut projectiles: Query<(&Transform, &mut Projectile, &mut Velocity)>) {

  for (enemy_transform, mut enemy_velocity, _) in enemies.iter_mut() {
    for (projectile_transform,mut projectile, mut velocity) in projectiles.iter_mut() {
      let distance = enemy_transform.translation.distance(projectile_transform.translation);

      if distance < 30.0 {
        println!("Enemy hit!");
        velocity.x = -velocity.x * 0.5;
        velocity.y = -velocity.y * 0.5;
        projectile.life = projectile.life * 0.5;

        enemy_velocity.x = -enemy_velocity.x * 0.5;
        enemy_velocity.y = -enemy_velocity.y * 0.5;
      }
    }
  }

}

pub fn enemy_targeting(
  mut enemies: Query<(&Enemy, &Transform, &mut Velocity)>, 
  player: Query<(&Player, &Transform
)>) {

  let player = player.single();

  let player_position = player.1.translation;

  for mut enemy in enemies.iter_mut() {
    let direction = player_position - enemy.1.translation;

    let direction = direction.normalize();

    enemy.2.x = direction.x * 100.0;
    enemy.2.y = direction.y * 100.0;
  }
}
