pub mod commands;
mod spatial;
use crate::commands::CommandRepo;
use crate::spatial::locations::{Location, Point};
use std::io;

pub struct Player {
    pub name: String,
    pub hp: i32,
    pub location: Location,
}

pub struct World {
    pub locations: Vec<Location>,
    pub player: Player,
}

pub struct Game {
    pub world: World,
    pub commands: CommandRepo,
}

impl Game {
    pub fn new(world: World, commands: CommandRepo) -> Game {
        let game: Game = Game { world, commands };
        game
    }
}

impl Player {
    pub fn new(name: String, location: Location) -> Player {
        let player: Player = Player {
            name,
            hp: 10,
            location: location,
        };
        player
    }
}

impl World {
    pub fn init_world() -> World {
        let world: World = World {
            locations: create_locations(),
            player: Player::new(
                String::from("Test"),
                Location {
                    name: String::from("test1"),
                    location: Point { x: 1, y: 0 },
                    description: String::from("test1"),
                },
            ),
        };
        world
    }
}

fn create_locations() -> Vec<Location> {
    let mut locations: Vec<Location> = Vec::new();

    locations.push(Location {
        name: String::from("Test"),
        location: Point { x: 0, y: 0 },
        description: String::from("Test"),
    });

    locations
}

pub fn get_input() -> String {
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Could not read line");

    answer
}
