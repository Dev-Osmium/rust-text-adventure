mod locations;
pub mod npcs {
    use crate::locations::locations::Location;

pub struct Npc {
        name: String,
        location: Location,
        description: String,
    }
    impl Npc {
        pub fn new(name: String, description: String, location: Location) -> Npc {
            let new_npc = Npc {
                name,
                description,
                location
            };
            new_npc
        }
    }
}