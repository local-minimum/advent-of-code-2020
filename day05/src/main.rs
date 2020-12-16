use std::fs;

// 1023 is highest possible value by BBBBBBBRRR    
const MAX_ID: usize = 1024;

#[derive(Debug)]
struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.col
    }

    fn from_string(data: &str) -> Option<Self> {
        let mut row: usize = 0;
        let mut col: usize = 0;
        let mut idx: usize = 0;
        let base: usize = 2;
        for chr in data.trim().chars() {
            match (chr, idx) {
                ('F', 0..=6) => {}, // Just ensuring it is valid
                ('B', 0..=6) => {row += base.pow((6 - idx) as u32)},
                ('L', 7..=9) => {}, // Just ensuring it is valid
                ('R', 7..=9) => {col += base.pow( (9 - idx) as u32)},
                _ => return None
            }
            idx += 1;
        }

        // Check that it's not truncated id string
        if idx != 10 { return None }

        Some(Seat {row, col})
    }

    fn from_id(id: usize) -> Option<Self> {
        match id {
            0..=MAX_ID => {
                let col = id % 8;
                let row = (id - col) / 8;
                Some(Seat {row, col})
            }
            _ => None
        }
    }
}

fn load_demo() -> String {
    r#"BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
BBBBBBBRRR
    "#.to_string()
}

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}


fn main() {
    let mut positions: [bool;MAX_ID] = [false; MAX_ID];
    // let data = load_demo();
    let data = load_data();
    let mut highest_id = 0;
    for line in data.lines() {
        match Seat::from_string(line) {
            Some(seat) => {
                println!("{:?}: {}", seat, seat.id());
                if seat.id() > highest_id {
                    highest_id = seat.id();
                }
                positions[seat.id()] = true;
            },
            None => {println!("Skipped row")},
        }
    }

    // Part 1 answer
    println!("\n*** Highest id {}\n", highest_id);

    // Debug from ID (bonus)
    println!("Validate from ID {:?}", Seat::from_id(76));
    println!("Validate from ID {:?}", Seat::from_id(518));
    println!("Validate from ID {:?}", Seat::from_id(905));

    // Part 2 answer
    let pattern: [bool; 3] = [true, false, true];
    let mut pidx = 0;
    for idx in 0..MAX_ID {
        if positions[idx] == pattern[pidx] {
            pidx += 1;
            if pidx == 3 {
                println!("\n*** Found seat id {} @ {:?}\n", idx - 1, Seat::from_id(idx - 1));
                break;
            }
        } else {
            pidx = 0;
        }
    }
}
