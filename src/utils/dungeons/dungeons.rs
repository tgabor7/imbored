use bevy::math::Vec2;
use rand::Rng;

static WIDTH: i32 = 500;
static HEIGHT: i32 = 500;

pub struct Room {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Room { x, y, width, height }
    }

    pub fn intersects(&self, other: &Room) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

impl Clone for Room {
    fn clone(&self) -> Self {
        Room {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

pub fn generate_rooms() -> Vec<Room> {
    let mut rng = rand::thread_rng();
    let mut rooms = Vec::new();

    for _ in 0..10 {
        let width = rng.gen_range(100..200);
        let height = rng.gen_range(100..200);
        let x = rng.gen_range(0..WIDTH - width);
        let y = rng.gen_range(0..HEIGHT - height);

        let new_room = Room::new(x, y, width, height);

        let mut failed = false;
        for room in &rooms {
            if new_room.intersects(room) {
                failed = true;
                break;
            }
        }

        if !failed {
            rooms.push(new_room);
        }
    }

    rooms
}

pub fn connect_rooms(rooms: &Vec<Room>) -> Vec<Room> {
    let mut rng = rand::thread_rng();
    let mut connected_rooms = Vec::from_iter(rooms.iter().cloned());

    for i in 0..rooms.len() {
        let room = &rooms[i];
        let next_room = &rooms[(i + 1) % rooms.len()];

        let start_x = rng.gen_range(room.x..room.x + room.width);
        let start_y = rng.gen_range(room.y..room.y + room.height);
        let end_x = rng.gen_range(next_room.x..next_room.x + next_room.width);
        let end_y = rng.gen_range(next_room.y..next_room.y + next_room.height);

        let mut x = start_x;
        let mut y = start_y;

        while x != end_x || y != end_y {
            if x != end_x {
                x += (end_x - x).signum();
            }

            if y != end_y {
                y += (end_y - y).signum();
            }

            connected_rooms.push(Room::new(x, y, 1, 1));
        }
    }

    connected_rooms
}
