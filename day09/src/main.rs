use std::collections::VecDeque;
use std::fs;

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

fn load_demo() -> String {
    r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#.to_string()
}

fn valid(window: &VecDeque<u64>, value: &u64) -> bool {
    for a in 0..window.len() - 1 {
        for b in a+1..window.len() {
            if window[a] + window[b] == *value {
                return true;
            }
        }
    }
    false
}

fn as_numbers(data: String) -> Vec<u64> {
    let mut parsed: Vec<u64> = Vec::new();
    for line in data.lines() {
        match line.parse::<u64>() {
            Ok(val) => {
                parsed.push(val);
            },
            Err(_) => {},
        }
    }
    parsed
}

fn scan_invalid(data: &Vec<u64>, preamble: usize) -> Option<u64> {
    let mut window: VecDeque<u64> = VecDeque::new();
    for val in data {
        if window.len() == preamble {
            if !valid(&window, &val) {
                return Some(val.clone());
            }
            window.pop_front();
        }
        window.push_back(val.clone());
    }
    None
}

fn scan_sums_to(data: &Vec<u64>, val: &u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    for i in 0..data.len() - 1 {
        let mut sum = data[i].clone();
        for j in i+1..data.len() {
            sum += data[j].clone();
            if sum == *val {
                for k in i..j+1 {
                    result.push(data[k]);
                }
                return result;
            } else if sum > *val {
                break;
            }
        }
    }
    result
}

fn main() {
    // let data = load_demo();
    // let preamble = 5;
    let data = load_data();
    let preamble = 25;
    let parsed = as_numbers(data);
    match scan_invalid(&parsed, preamble) {
        Some(val) => {
            println!("Part 1: {}", val);
            let sums = scan_sums_to(&parsed, &val);
            println!("\n{:?}", sums);
            println!("Part 2: {}", sums.iter().min().unwrap() + sums.iter().max().unwrap());
        },
        None => {}
    }
}
