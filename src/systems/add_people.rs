use bevy::{
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::Commands,
    math::{Vec2, Vec3},
    prelude::TransformBundle,
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};
use bevy_rapier2d::prelude::{
    AdditionalMassProperties, Collider, ExternalForce, GravityScale, KinematicCharacterController,
    Restitution, RigidBody,
};

use crate::{
    components::{
        acceleration::Acceleration, enemy::Enemy, main_camera::MainCamera, physics::Physics,
        player::Player, velocity::Velocity,
    },
    utils::dungeons::dungeons::{self},
};

pub fn add_dungeon(commands: &mut Commands) {
    let rooms = dungeons::generate_rooms();
    let connected_rooms = dungeons::connect_rooms(&rooms);

    for room in connected_rooms {
        let x = room.x as f32;
        let y = room.y as f32;
        let width = room.width as f32;
        let height = room.height as f32;

        commands
            .spawn(Collider::cuboid(width / 2.0, height / 2.0))
            .insert(TransformBundle::from(Transform::from_xyz(x, y, 0.0)));
    }
}

pub fn add_people(mut commands: Commands) {
    add_dungeon(&mut commands);

    commands
        .spawn(Collider::cuboid(500.0, 30.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
        .insert(Enemy {
            target: false,
            health: 100,
        });

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(30.0, 30.0))
        .insert(Player { can_move: true })
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
        .insert(KinematicCharacterController::default());

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(30.0, 30.0))
        .insert(GravityScale(0.))
        .insert(AdditionalMassProperties::Mass(1000.))
        .insert(Enemy {
            health: 10,
            target: false,
        })
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(
            400.0, 400.0, 0.0,
        )));

    commands.spawn((Camera2dBundle::default(), MainCamera));
}
