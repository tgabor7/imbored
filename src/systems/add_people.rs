use bevy::{ecs::system::Commands, sprite::{SpriteBundle, Sprite}, render::color::Color, utils::default, transform::components::Transform, math::{Vec3, Vec2}, core_pipeline::core_2d::Camera2dBundle};

use crate::components::player::Player;

pub fn add_people(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    }, Player));
}
