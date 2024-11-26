use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Player {
  pub can_move: bool,
}
