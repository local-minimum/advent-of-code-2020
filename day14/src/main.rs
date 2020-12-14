use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn load_demo() -> String {
    r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#.to_string()
}

fn load_demo2() -> String {
    r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#.to_string()
}

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

fn get_register(re: &Regex, operation: &str) -> usize {
    let res = re.captures(operation).unwrap();
    res[1].parse().unwrap()
}

const BASE: usize = 2;
fn to_string_form(mut value: usize) -> String {
    
    let mut data: String = "".to_string();
    for idx in (0..36).rev() {
        let v = BASE.pow(idx);
        if value >= v {
            data.push('1');
            value -= v;
        } else {
            data.push('0');
        }
    }
    data
}

fn to_numeric_form(mut value: String) -> usize {
    let mut result = 0;
    for idx in (0..36).rev() {
        match value.pop() {
            Some(v) => {
                match v {
                    '1' => result += BASE.pow(35 - idx),
                    '0' => {},
                    _ => panic!("Unhandled value in {}", v),
                }
            },
            None => panic!("Value too short ({} long)", idx + 1),
        }
    }
    result
}

fn apply_mask(value: &String, mask: &str) -> String {
    let mut result: String = "".to_string();
    let mask_chars: Vec<char> = mask.chars().collect();
    let value_chars: Vec<char> = value.chars().collect();
    for idx in 0..value.len() {
        match mask_chars[idx] {
            '0' => result.push('0'),
            '1' => result.push('1'),
            'X' => result.push(value_chars[idx]),
            _ => panic!("Bad character in {}", mask),
        }
    }
    result
}

fn sum_memory(memory: &HashMap<usize, usize>) -> usize {
    let mut acc = 0;
    for (_, value) in memory {
        acc += value;
    }
    acc
}

fn run_model_one(
    mask: &str,
    value: &usize,
) -> usize {
    let s_value = to_string_form(value.clone());
    let result = apply_mask(&s_value, mask);
    let result_value = to_numeric_form(result.clone());
    result_value
}

fn recursive_perms(data: &Vec<char>, start: usize) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for idx in start..36 {
        if idx == 35 {
            if data[idx] == 'X' {
                let mut new_data = data.clone();
                new_data[idx] = '1';
                res.push(new_data.iter().collect());
                new_data[idx] = '0';
                res.push(new_data.into_iter().collect());

            } else {
                res.push(data.into_iter().collect())
            }
        }
        if data[idx] == 'X' {
            let mut new_data = data.clone();
            new_data[idx] = '1';
            res.append(&mut recursive_perms(&new_data, idx + 1));
            new_data[idx] = '0';
            res.append(&mut recursive_perms(&new_data, idx + 1));
            return res;
        }
    }
    res
}

fn run_model_two(
    mask: &str,
    register: &usize,
) -> Vec<usize> {
    let reg = to_string_form(register.clone());
    let mask_chars: Vec<char> = mask.chars().collect();
    let mut reg_chars: Vec<char> = reg.chars().collect();
    for idx in 0..36 {
        match mask_chars[idx] {
            '0' => {},
            '1' => reg_chars[idx] = '1',
            'X' => reg_chars[idx] = 'X',
            _ => panic!("Unhandled character in {}", mask),
        }
    }
    let mut res: Vec<usize> = Vec::new();
    // println!("re {}", reg);
    for s_reg in recursive_perms(&reg_chars, 0) {
        // println!("-> {}", s_reg);
        res.push(to_numeric_form(s_reg));
    }
    res
}

fn main() {
    let re = Regex::new(r"mem\[(\d+)\]").unwrap();
    // let data = load_demo();
    // let data = load_demo2();
    let data = load_data();
    let mut mask: &str = "";
    let mut memory_one: HashMap<usize, usize> = HashMap::new();
    let mut memory_two: HashMap<usize, usize> = HashMap::new();
    for line in data.lines() {
        let mut split = line.split("=");
        let operation = split.next().unwrap().trim();
        let value = split.next().unwrap().trim();
        if operation == "mask" {
            // println!("ma {}", value);
            mask = value;
        } else {
            let u_value: usize = value.parse().unwrap();
            let register = get_register(&re, operation);
            let outcome = run_model_one(mask, &u_value);
            memory_one.insert(register, outcome);
            for reg in run_model_two(mask, &register) {
                // println!("{}\t{}", reg, u_value);
                memory_two.insert(reg, u_value);
            }
        }
    }
    println!("Part 1: Memory sum {}", sum_memory(&memory_one));
    println!("Part 2: Memory sum {}", sum_memory(&memory_two));
}
