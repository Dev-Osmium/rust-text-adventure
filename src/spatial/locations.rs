pub struct Location {
    pub name: String,
    pub location: Point,
    pub description: String,
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Location {
    pub fn new(name: String, location: Point, description: String) -> Location {
        let new_location: Location = Location {
            name,
            location,
            description,
        };
        new_location
    }
    pub fn get_location(&self) -> (i32, i32) {
        (self.location.x, self.location.y)
    }
    pub fn print_location(&self) {
        println!(
            "({},{})\n{}",
            self.location.x, self.location.y, self.description
        );
    }
}
