fn load_demo() -> String {
    "389125467".to_string()
}

fn load_data() -> String {
    "586439172".to_string()
}

#[derive(Debug)]
struct Game {
    cups: Vec<usize>,
    pick_up: Vec<usize>,
    move_id: usize,
    verbose: bool,    
    focus: usize,
    max_cup: usize,
    n_cups: usize,
}

impl Game {    
    fn from_string(data: String, verbose: bool) -> Game {
        let mut cups: Vec<usize> = vec![];
        for c in data.chars() {
            cups.push(c.to_string().parse().unwrap());
        }
        let max_cup = cups.iter().max().unwrap().clone();
        let n_cups = cups.len();
        Game{cups, pick_up: vec![0, 0, 0], move_id: 0, verbose, focus: 0, max_cup, n_cups}
    }

    fn add_part_two_cups(&mut self) {
        let start = self.cups.iter().max().unwrap();
        let end = 1000000 - self.cups.len() + start + 1;
        self.cups.extend(start+1..end);
        self.max_cup = self.cups.iter().max().unwrap().clone();
        self.n_cups = self.cups.len();
    }

    fn print_cups(&self) {
        println!(
            "cups: {}",
            self.cups
                .iter()
                .enumerate()                
                .map(|(idx, c)| format!("{} ", match idx == self.focus { true => format!("({})", c), false => c.to_string()}))
                .collect::<String>(),
        );
    }

    fn do_move(&mut self) {
        self.move_id += 1;
        if self.verbose {
            println!("-- move {} --", self.move_id);
            self.print_cups();
        }
        let current = self.cups[self.focus];
        let mut pick_up_pos = self.focus + 1;
        for idx in 0..3 {
            if pick_up_pos == self.n_cups - idx { 
                pick_up_pos = 0;
            }
            self.pick_up[idx] = self.cups.remove(pick_up_pos);
        }
        let mut destination: usize  = match current == 1 {
            true => self.max_cup,
            false => current - 1,
        };
        while self.pick_up.contains(&destination) {
            destination = match destination == 1 {
                true => self.max_cup,
                false => destination - 1,
            };
        }
        if self.verbose {
            println!("pick up: {} {} {}", self.pick_up[0], self.pick_up[1], self.pick_up[2]);
            println!("destination: {}\n", destination);
        }
        let pos = self.cups.iter().position(|c| *c == destination).unwrap();
        for idx in 0..3 {
            self.cups.insert(pos + idx + 1, self.pick_up[idx]);
        }
        self.focus = self.cups.iter().position(|c| *c == current).unwrap();
        self.focus += 1;
        if self.focus == self.n_cups {
            self.focus = 0;
        }
    }

    fn score(&self) {
        if self.verbose {
            println!("-- final --");
            self.print_cups();
        }
        let pos = self.cups.iter().position(|c| *c == 1).unwrap();
        let right: String = self.cups[pos+1..].iter().map(|c| c.to_string()).collect();
        let left: String = self.cups[..pos].iter().map(|c| c.to_string()).collect();
        println!("{}{}", right, left);
    }

    fn score_part2(&self) {
        let mut pos = self.cups.iter().position(|c| *c == 1).unwrap() + 1;
        if pos > self.cups.len() {
            pos = 0;
        }
        let  a = self.cups[pos].clone();
        pos += 1;
        if pos > self.cups.len() {
            pos = 0;
        }
        let b = self.cups[pos].clone();
        println!("Star cups are {} and {}, key is {}", a, b, a*b);
    }
}

fn main() {
    let part_two = true;
    let is_intro = false;
    let is_demo = true;
    let data = match is_demo { true => load_demo(), false => load_data() };
    let mut game = Game::from_string(data, !part_two);
    let rounds = match (is_intro, part_two) { 
        (false, true) => 10000000,
        (false, false) => 100,
        _ => 10,
    };
    if part_two {
        game.add_part_two_cups();
    }
    println!("Game has {} cups", game.cups.len());
    while game.move_id < rounds {
        game.do_move();
        if game.move_id % 100 == 0 && part_two {
            println!("Move {}", game.move_id);
        }
    } 
    if part_two {
        game.score_part2();
    } else {
        game.score();   
    }
}
