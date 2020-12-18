use std::fs;

fn load_demo() -> String {
    r#"1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"#.to_string()
}

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

fn load_demo_expect() -> Vec<usize> {
    vec![
        71,
        51,
        26,
        437,
        12240,
        13632,
    ]
}

fn load_demo_expect2() -> Vec<usize> {
    vec![
        231,
        51,
        46,
        1445,
        669060,
        23340,
    ]
}

enum Op {None, Add, Mul}

fn eval_eq_prio(line: &str) -> usize {
    let mut total = 0;
    let mut op = Op::None;
    for part in line.split(" ") {
        match part.parse::<usize>() {
            Ok(val) => {
                match op {
                    Op::None => total = val,
                    Op::Add => total += val,
                    Op::Mul => total *= val,
                }
            },
            Err(_) => {
                if part == "+" {
                    op = Op::Add;
                } else if part == "*" {
                    op = Op::Mul;
                } else {
                    panic!("Warning: '{}' not understood in: {}", part, line);
                }
            }
        }
    }
    total
}

fn eval_add_prio(line: &str) -> usize {
    let mut parts: Vec<String> = line.split(" ").map(|v| v.to_string()).collect();
    let mut idx = 1;
    loop {
        if parts[idx] == "+" {
            let val:usize = parts[idx -1].parse::<usize>().unwrap() + parts[idx + 1].parse::<usize>().unwrap();
            let sval = val.to_string();
            parts = parts
                .iter()
                .enumerate()
                .map(|(id, v)| {
                    if id < idx - 1 {return (v, true)};
                    if id > idx + 1 {return (v, true)};
                    if id == idx {return (&sval, true)}
                    return (v, false);
                })
                .filter(|(_, keep)| *keep)
                .map(|(v, _)| v.clone())
                .collect();
            idx -= 1;
        }
        idx +=1;
        if idx >= parts.len() {
            break;
        }
    }
    parts
        .into_iter()
        .filter(|v| v != "*")
        .fold(1, |a, b| a * b.parse::<usize>().unwrap())
}

fn calculate(line: &str, eval: fn(line: &str) -> usize) -> usize {
    let mut openings: Vec<usize> = vec![];
    let mut data: String = line.to_string();
    let mut idx = 0;
    loop {
        let chars: Vec<char> = data.chars().collect();
        match chars[idx] {
            '(' => {
                openings.push(idx);
            },
            ')' => {
                let start = openings.pop().unwrap();
                let part = &data[start+1..idx];
                // println!("{} of {}", part, data);
                let val = eval(part);
                data = format!("{}{}{}" , &data[..start], val, &data[idx+1..]);
                idx -= idx - start;
                // println!("{}, {:?}, {}, {}", data, openings, idx, &data[idx..]);
            },
            _ => {
            }
        }
        idx+=1;
        if idx >= data.len() {
            break;
        }
    }
    eval(&data)
}

fn main() {
    let demo = false;
    let part1: bool = false;
    let data = match demo { true => load_demo(), false => load_data()};
    let expect = match part1 { true => load_demo_expect(), false => load_demo_expect2()};
    let eval = match part1 { true => eval_eq_prio, false => eval_add_prio};
    let mut sum = 0;
    for (idx, line) in data.lines().enumerate() {
        let val = calculate(line, eval);
        if demo {
            match val == expect[idx] {
                true => {
                    println!(" OK: {} == {}", line, val);
                },
                false => {
                    println!("NOK: {} != {} ( == {})", line, val, expect[idx]);
                }
            }
        }
        sum += val;
    }
    println!("Sum is: {}", sum);
}
