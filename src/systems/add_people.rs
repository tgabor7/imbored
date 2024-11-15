use bevy::{
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::Commands,
    math::{Vec2, Vec3},
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use crate::components::{
    acceleration::Acceleration, enemy::Enemy, main_camera::MainCamera, physics::Physics,
    player::Player, velocity::Velocity,
};

pub fn add_people(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.95, 0.05, 0.05),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 100., 0.)),
            ..default()
        },
        Enemy,
        Velocity { x: 0., y: 0. },
        Physics { friction: 0.9 },
        Acceleration { x: 0., y: 0. },
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            ..default()
        },
        Player,
        Velocity { x: 10., y: 0. },
        Physics { friction: 0.9 },
        Acceleration { x: 0., y: 0. },
    ));
}
