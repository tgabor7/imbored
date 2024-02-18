use bevy::{app::{App, Update, Startup}, DefaultPlugins};
use systems::{hello_world::hello_world, greet_people::greet_people, player_movement::player_movement};

mod systems;
mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, systems::add_people::add_people)
        .add_systems(Update, (greet_people, player_movement))
        .run();
}
