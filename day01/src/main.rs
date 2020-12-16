use std::fs;
use std::collections::HashMap;
use combinations::Combinations;

fn read_file() -> String {
    let contents = fs::read_to_string("./input.txt")
        .expect("Could not load data");
    return contents
}

fn load_data() -> Vec<i32> {
    let raw: Vec<i32> = read_file()
        .lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .collect();
    return raw    
}

fn main() {
    let values = load_data();
    let n = values.len();
    // Part 1
    for x in 0..n {
        for y in x+1..n {
            if values[x] + values[y] == 2020 {
                println!("{} + {} = 2020", values[x], values[y]);
                println!("{} * {} = {}", values[x], values[y], values[x] * values[y]);
            }
        }
    }
        
    // Part 2
    for x in 0..n {
        for y in x+1..n {
            for z in y+1..n {
                if values[x] + values[y] + values[z] == 2020 {
                    println!("{} + {} + {} = 2020", values[x], values[y], values[z]);
                    println!("{} * {} * {} = {}", values[x], values[y], values[z], values[x] * values[y] * values[z]);
                }

            }
        }
    }

    // Part 1 & 2 version 2
    for n in 2..4 {
        let result: HashMap<i32, Vec<i32>> = Combinations::new(values.clone(), n)
            .filter(|v| (v.iter().fold(0, |a, b| a + *b) == 2020))
            .map(|v| (v.iter().fold(1, |a, b| a * *b), v.clone()))
            .collect();

        for (key, val) in result.into_iter() {
            println!("{}: {} {:?}", n, key, val);
        }
    }
}
