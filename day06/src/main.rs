use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

fn load_demo() -> String {
    r#"abc

a
b
c

ab
ac

a
a
a
a

b
    "#.to_string()
}

fn part_one_rule(group: &str) -> usize {
    let line = group.replace("\n", "");
    let mut set: HashSet<char> = HashSet::new();
    for ch in line.chars() {
        match ch {
            'a'..='z' => { set.insert(ch); },
            _ => {},
        }
    }
    set.len()
}

fn part_two_rule(group: &str) -> usize {
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut people: usize = 0;
    for person in group.lines() {
        let line = person.trim();
        if line.len() == 0 {
            continue;
        }
        for ch in line.chars() {
            match ch {
                'a'..='z' => {
                    match map.get(&ch) {
                        Some(v) => map.insert(ch, v + 1),
                        None => map.insert(ch, 1),
                    };
                },
                _ => {},
            }
        }
        people += 1;
    }
    map
        .iter()
        .filter(|(_, v)| v == &&people)
        .count()
}

fn main() {
    // let data = load_demo();
    let data = load_data();
    let mut total_pt1 = 0;
    let mut total_pt2 = 0;
    for group in data.split("\n\n") {
        let pt1 = part_one_rule(group);
        let pt2 = part_two_rule(group);
        println!("------------\n{}\nResult: {}\t{}", group.trim(), pt1, pt2);
        total_pt1 += pt1;
        total_pt2 += pt2;
    }
    println!("\nTotals: {}\t{}", total_pt1, total_pt2);
}
