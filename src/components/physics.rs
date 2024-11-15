use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Physics {
    pub friction: f32,
}
