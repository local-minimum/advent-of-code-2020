use std::fs;

fn load_demo() -> String {
    r#"F10
N3
F7
R90
F11"#.to_string()
}

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

fn parse_instruction(instruction: &str) -> Option<(char, i64)> {
    match instruction.get(..1) {
        Some(ch_str) => {
            let ch: char = ch_str.chars().next().unwrap(); 
            match instruction.get(1..) {
                Some(val) => {
                    match val.parse::<i64>() {
                        Ok(amount) => Some((ch , amount)),
                        Err(_) => None,
                    }
                },
                None => None,
            }
        },
        None => None 
    }
}

struct Point {
    lat: i64,
    lon: i64,
}

impl Point {
    fn origin() -> Self {
        Point{lat: 0, lon: 0}
    }

    fn manhattan(&self) -> i64 {
        self.lat.abs() + self.lon.abs()        
    }

    fn translate(&mut self, direction: Point) {
        self.lon += direction.lon;
        self.lat += direction.lat;
    }

    fn rotate(&mut self, amount: i64) {
        let mut rotation = amount;
        while rotation != 0 {
            if rotation.abs() < 90 {
                panic!("Rotation not per 90 degrees {}", amount);
            }
            if rotation < 0 {
                let lon = self.lat;
                self.lat = -self.lon;
                self.lon = lon;
                rotation += 90;
            } else {
                let lon = -self.lat;
                self.lat = self.lon;
                self.lon = lon;
                rotation -= 90;
            }
        }
    }

    fn scaled(&self, magnitude: i64) -> Point {
        Point{lat: self.lat * magnitude, lon: self.lon * magnitude}
    }

    fn print(&self) {
        let ew = match self.lon > 0 {
            true => "east",
            false => "west",
        };
        let nw = match self.lat > 0 {
            true => "north",
            false => "south",
        };
        println!("{} {}, {} {}", ew, self.lon, nw, self.lat);
    }
}

struct Boat {
    pos: Point,
    heading: Point,
}

impl Boat {
    fn origin() -> Self {
        Boat{pos: Point::origin(), heading: Point{lat: 0, lon: 1}}
    }

    fn enact(&mut self, instruction: &str) {
        match parse_instruction(instruction) {
            Some((kind, amount)) => {
                match kind {
                    'N' => self.pos.translate(Point{ lat: amount, lon: 0}),
                    'S' => self.pos.translate(Point{ lat: -amount, lon: 0}),
                    'E' => self.pos.translate(Point{ lat: 0, lon: amount}),
                    'W' => self.pos.translate(Point{ lat: 0, lon: -amount}),
                    'F' => self.pos.translate(self.heading.scaled(amount)),
                    'L' => self.heading.rotate(amount),
                    'R' => self.heading.rotate(-amount),
                    _ => {},
                }
            },
            None => {},
        }
    }
}

struct Boat2  {
    pos: Point,
    waypoint: Point,
}

impl Boat2 {
    fn origin() -> Self {
        Boat2{pos: Point::origin(), waypoint: Point{lat: 1, lon: 10}}
    }

    fn enact(&mut self, instruction: &str) {
        match parse_instruction(instruction) {
            Some((kind, amount)) => {
                match kind {
                    'N' => self.waypoint.translate(Point{ lat: amount, lon: 0}),
                    'S' => self.waypoint.translate(Point{ lat: -amount, lon: 0}),
                    'E' => self.waypoint.translate(Point{ lat: 0, lon: amount}),
                    'W' => self.waypoint.translate(Point{ lat: 0, lon: -amount}),
                    'F' => self.pos.translate(self.waypoint.scaled(amount)),
                    'L' => self.waypoint.rotate(amount),
                    'R' => self.waypoint.rotate(-amount),
                    _ => {},
                }
            },
            None => {},
        }
    }
}

fn main() {
    // let data = load_demo();
    let data = load_data();
    let mut boat = Boat::origin();
    let verbose = false;
    for line in data.lines() {
        boat.enact(line);
        if verbose {
            boat.pos.print();
        }
    }
    println!("\nPart 1: Manhattan distance travelled: {}", boat.pos.manhattan());
    let mut boat2 = Boat2::origin();
    for line in data.lines() {
        boat2.enact(line);
        if verbose {
            println!("-----");
            boat2.waypoint.print();
            boat2.pos.print();
        }
    }
    println!("\nPart 2: Manhattan distance travelled: {}", boat2.pos.manhattan());
}
