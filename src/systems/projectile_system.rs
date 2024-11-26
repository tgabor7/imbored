use bevy::{ecs::{query::BatchingStrategy}, prelude::*};

use crate::components::{projectile::Projectile, velocity::Velocity};

pub fn projectile_system(mut query: Query<(&mut Projectile)>,time: Res<Time>) {

    let delta = time.delta_seconds();

    query.par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(32))
        .for_each(|(mut projectile)| {
            projectile.life -= delta;
        });

}

pub fn projectile_despawn_system(mut commands: Commands, query: Query<(Entity, &Projectile)>) {
    for (entity, projectile) in query.iter() {
        if projectile.life <= 0. {
            commands.entity(entity).despawn();
        }
    }
}
