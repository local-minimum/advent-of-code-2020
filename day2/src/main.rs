use std::fs;
use regex::Regex;

struct Pass {
    pwd: String,
    low: usize,
    high: usize,
    val: char,
}


fn read_data() -> Vec<Pass> {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    let lines = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| re.captures(l.trim()).unwrap())
        .map(|c| Pass {
            pwd: String::from(&c[3]),
            val: c[2].chars().next().unwrap(),
            low: c[0].parse::<usize>().unwrap(),
            high: c[1].parse::<usize>().unwrap(),
        })
        .collect();
    return lines
}

trait Ranged {
    fn in_range(&self, low: usize, high: usize) -> bool;
}

impl Ranged for usize {
    fn in_range(&self, low: usize, high: usize) -> bool {
        return *self >= low && *self <= high
    }
}

fn main() {
    let lines = read_data();
    let filtered = lines
        .iter()
        .filter(|v| v.pwd.matches(v.val).count().in_range(v.low, v.high));

    for line in filtered {
        println!("{} has {} - {} {}", line.pwd, line.val, line.low, line.high);
    }
    
}
