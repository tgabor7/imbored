use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32,
}
