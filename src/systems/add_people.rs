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
    Collider, ExternalForce, KinematicCharacterController, Restitution, RigidBody,
};

use crate::components::{
    acceleration::Acceleration, enemy::Enemy, main_camera::MainCamera, physics::Physics,
    player::Player, velocity::Velocity,
};

pub fn add_people(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(500.0, 30.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(30.0, 30.0))
        .insert(Player { can_move: true })
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
        .insert(KinematicCharacterController::default());

    commands.spawn((Camera2dBundle::default(), MainCamera));
    // commands.spawn((
    //     SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::rgb(0.95, 0.05, 0.05),
    //             ..default()
    //         },
    //         transform: Transform::from_translation(Vec3::new(0., -100., 10.)).with_scale(Vec3::new(300., 30., 1.)),
    //         ..default()
    //     },
    //     Enemy { target: false },
    //     Collider::cuboid(300.0, 30.0),
    //     Velocity { x: 0., y: 0. },
    //     Physics { friction: 0.9 },
    //     Acceleration { x: 0., y: 0. },
    // ));
    // commands.spawn((
    //     SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::rgb(0.95, 0.05, 0.05),
    //             custom_size: Some(Vec2::new(40.0, 40.0)),
    //             ..default()
    //         },
    //         transform: Transform::from_translation(Vec3::new(0., 100., 0.)),
    //         ..default()
    //     },
    //     Enemy { target: true },
    //     Velocity { x: 0., y: 0. },
    //     Physics { friction: 0.9 },
    //     Acceleration { x: 0., y: 0. },
    // ));
    // commands.spawn((
    //     SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::rgb(0.25, 0.25, 0.75),
    //             ..default()
    //         },
    //         transform: Transform::from_translation(Vec3::new(-50., 0., 0.)).with_scale(Vec3::new(30., 30., 1.)),
    //         ..default()
    //     },
    //     Player { can_move: true },
    //     Collider::cuboid(30.0, 30.0),
    //     Velocity { x: 10., y: 0. },
    //     Physics { friction: 0.9 },
    //     Acceleration { x: 0., y: 0. },
    // ));
}
