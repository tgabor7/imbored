use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Projectile {
    pub life: f32,
    pub damage: i32,
}
