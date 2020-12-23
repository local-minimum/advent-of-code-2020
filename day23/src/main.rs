fn load_demo() -> String {
    "389125467".to_string()
}

fn load_data() -> String {
    "586439172".to_string()
}

#[derive(Debug)]
struct Game {
    cups: Vec<usize>,
    move_id: usize,
    verbose: bool,    
    focus: usize,
    max_cup: usize,
}

impl Game {    
    fn from_string(data: String, verbose: bool) -> Game {
        let mut cups: Vec<usize> = (0..data.len()+1).into_iter().collect();
        let chars: Vec<char> = data.chars().collect();
        let first: usize = chars[0].to_string().parse().unwrap();
        let mut previous = first.clone();
        for c in chars[1..].iter() {
            let v = c.to_string().parse().unwrap();
            cups[previous] = v;
            previous = v;
        }
        cups[previous] = first;
        let max_cup = cups.len() - 1;
        Game{cups, move_id: 0, verbose, focus: first, max_cup}
    }

    fn add_part_two_cups(&mut self) {
        let start_target = self.max_cup+2;
        let end = 1000000 - self.cups.len() + start_target + 1;
        self.cups.extend(start_target..end);
        let remap = self.cups.iter().position(|c| *c == self.focus).unwrap();
        self.cups[remap] = self.max_cup + 1;         
        self.max_cup = 1000000;
        self.cups[self.max_cup] = self.focus;
    }

    fn print_cups(&self) {
        let mut next = self.focus;
        print!("cups: ({})", next);
        loop {
            next = self.cups[next];
            if next == self.focus {
                break;
            } else {
                print!(" {}", next);
            }
        }
        println!();
    }

    fn do_move(&mut self) {
        self.move_id += 1;
        let p1 = self.cups[self.focus];
        let p2 = self.cups[p1];
        let p3 = self.cups[p2];
        let cont = self.cups[p3];
        let mut destination: usize  = match self.focus == 1 {
            true => self.max_cup,
            false => self.focus - 1,
        };
        while destination == p1 || destination == p2 || destination == p3 {
            destination = match destination == 1 {
                true => self.max_cup,
                false => destination - 1,
            };
        }
        if self.verbose {
            println!("-- move {} --", self.move_id);
            self.print_cups();
            println!("pick up: {} {} {}", p1, p2, p3);
            println!("destination: {}\n", destination);
        }
        self.cups[self.focus] = cont;
        let cont2 = self.cups[destination];
        self.cups[destination] = p1;
        self.cups[p3] = cont2;
        self.focus = self.cups[self.focus];
    }

    fn score(&self) {
        if self.verbose {
            println!("-- final --");
            self.print_cups();
        }
        let mut solution = "".to_string();
        let mut pos = 1;
        loop {
            pos = self.cups[pos];
            if pos == 1 { break; }
            solution = format!("{}{}", solution, pos);
        }
        println!("{}", solution);
    }

    fn score_part2(&self) {
        let a = self.cups[1];
        let b = self.cups[a];
        println!("Star cups are {} and {}, key is {}", a, b, a*b);
    }
}

fn main() {
    let part_two = true;
    let is_intro = false;
    let is_demo = false;
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
    println!("Game has {} cups", game.cups.len() - 1);
    while game.move_id < rounds {
        game.do_move();
    } 
    if part_two {
        game.score_part2();
    } else {
        game.score();   
    }
}
