use bevy::{ecs::query::BatchingStrategy, prelude::*}; 
use crate::components::{acceleration::Acceleration, physics::Physics, velocity::Velocity};

pub fn physics_system(mut query: Query<(&mut Transform, &mut Velocity, &Acceleration, &Physics)>,time: Res<Time>) {

    let delta = time.delta_seconds();

    query.par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(32))
        .for_each(|(mut transform, velocity, _, _physics)| {
            transform.translation.x += velocity.x * delta;
            transform.translation.y += velocity.y * delta;
        });

    query.par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(32))
        .for_each(|(_, mut velocity, _, physics)| {
            velocity.x *= physics.friction;
            velocity.y *= physics.friction;
        });

    query.par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(32))
        .for_each(|(_, mut velocity, acceleration, _physics)| {
            velocity.x += acceleration.x * delta;
            velocity.y += acceleration.y * delta;
        });
}
