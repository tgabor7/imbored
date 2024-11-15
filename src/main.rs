use bevy::{app::{App, Update, Startup}, DefaultPlugins};
use systems::{camera_system::camera_system, enemy_system::{enemy_collision, enemy_targeting}, greet_people::greet_people, physics::physics_system, player_movement::player_movement, projectile_system::{projectile_despawn_system, projectile_system}};

mod systems;
mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, systems::add_people::add_people)
        .add_systems(Update, (
                greet_people, 
                player_movement,
                physics_system,
                projectile_system,
                camera_system,
                projectile_despawn_system,
                enemy_targeting,
                enemy_collision

            ))
        .run();
}
