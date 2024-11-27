use bevy::ecs::{query::With, system::Query};

use crate::components::{name::Name, player::Player};

pub fn greet_people(_query: Query<&Name, With<Player>>) {
    // for name in query.iter() {
    //     println!("Hello, {}!", name.0);
    // }
}
