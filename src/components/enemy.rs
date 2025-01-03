use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Enemy {
    pub target: bool,
    pub health: i32,
}
