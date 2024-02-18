use bevy::ecs::{query::With, system::Query};

use crate::components::{player::Player, name::Name};

pub fn greet_people(query: Query<&Name, With<Player>>) {
    // for name in query.iter() {
    //     println!("Hello, {}!", name.0);
    // }
}
