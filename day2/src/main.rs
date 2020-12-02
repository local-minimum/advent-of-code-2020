use std::fs;
use regex::Regex;

fn read_data() -> String {
    let lines = fs::read_to_string("./input.txt")
        .unwrap();
    return lines
}

trait InRanged {
    fn in_range(&self, low: usize, high: usize) -> bool;
}

impl InRanged for usize {
    fn in_range(&self, low: usize, high: usize) -> bool {
        return *self >= low && *self <= high
    }
}

trait Password {
    fn valid(&self, chr: char, idx1: usize, idx2: usize) -> bool;
}

impl Password for String {
    fn valid(&self, chr: char, idx1: usize, idx2: usize) -> bool {
        let mut matches = 0;
        let mut chars = self.chars();
        for idx in 1..idx2 + 1 {
            let c = chars.next().unwrap();
            if idx == idx1 && c == chr {
                matches += 1;
            } else if idx == idx2 && c == chr {
                matches += 1;
            }
        }
        return matches == 1;
    }
}

fn main() {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    let data = read_data();

    // Part 1 & Part 2
    let mut valid_part1: usize = 0;
    let mut valid_part2: usize = 0;
    for line in data.lines().map(|l| l.trim()) {
        let caps = re.captures(line)
            .expect("Could not parse line");
        let pwd =  String::from(&caps[4]);
        let chr = caps[3].chars().next().unwrap();
        let low = caps[1].parse::<usize>().unwrap();
        let high = caps[2].parse::<usize>().unwrap();
        if pwd.matches(chr).count().in_range(low, high) {
            valid_part1+=1;
        }
        if pwd.valid(chr, low, high) {
            valid_part2+=1;
        }
    }
    println!("{} valid passwords part 1", valid_part1);
    println!("{} valid passwords part 2", valid_part2);
}
