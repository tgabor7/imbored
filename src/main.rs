use bevy::{
    app::{App, Startup, Update},
    DefaultPlugins,
};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use systems::{
    camera_system::camera_system,
    enemy_system::{enemy_collision, enemy_despawn_system},
    greet_people::greet_people,
    physics::physics_system,
    player_movement::player_movement,
    projectile_system::{projectile_despawn_system, projectile_system},
};

mod components;
mod systems;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, systems::add_people::add_people)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(
            Update,
            (
                // greet_people,
                player_movement,
                projectile_system,
                projectile_despawn_system,
                enemy_collision,
                enemy_despawn_system
            ),
        )
        .run();
}
