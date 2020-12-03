use std::fs;

fn read_file() -> String {
    return fs::read_to_string("./input.txt").unwrap()
}

struct Slope {
    step_x: usize,
    step_y: i32,
    trees: u64,
    x: usize,
    y: i32,
}

impl Slope {
    fn move_down(&mut self) {
        self.y += self.step_y;
        self.x += self.step_x;
    }

    fn align(&mut self, row_len: usize) {
        self.x %= row_len;
    }

    fn tree(&mut self, row: &str) {
        let pos = row.get(self.x..self.x+1).unwrap();
        match pos {
            "." => println!("{},{} FREE: ", self.step_x, self.step_y),
            "#" => {println!("{},{} TREE: ", self.step_x, self.step_y); self.trees+=1;},
            _ => println!("{},{} ERROR with {}:", self.step_x, self.step_y, pos),
        }
    }

    fn active(&mut self, y: i32) -> bool {
        return match self.y == y {
            true => true,
            false => {println!("{},{} SkIP LINE", self.step_x, self.step_y); return false},
        }
    }
}

fn main() {
    // Puzzle 1 & 2
    let mut slopes: [Slope; 5] = [
        Slope {step_x: 1, step_y: 1, trees: 0, x: 0, y: 0},
        Slope {step_x: 3, step_y: 1, trees: 0, x: 0, y: 0},
        Slope {step_x: 5, step_y: 1, trees: 0, x: 0, y: 0},
        Slope {step_x: 7, step_y: 1, trees: 0, x: 0, y: 0},
        Slope {step_x: 1, step_y: 2, trees: 0, x: 0, y: 0},
    ];
    let mut y = 0;
    let data = read_file();
    for line in data.lines() {
        let row = line.trim();
        let row_len = row.len();
        for slope in slopes.iter_mut() {
            if slope.active(y) {
                slope.align(row_len);
                slope.tree(row);
                slope.move_down();
            }
        }
        y+=1;
    }
    let mut product_of_trees = 1;
    for slope in slopes.iter() {
        product_of_trees *= slope.trees;
        println!("{}, {} gave {} trees", slope.step_x, slope.step_y, slope.trees);
    }
    println!("Product of trees {}", product_of_trees);
}
