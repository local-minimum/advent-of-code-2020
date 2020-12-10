use std::fs;

fn load_demo() -> String {
    r#"16
10
15
5
1
11
7
19
6
12
4"#.to_string()
}

fn load_demo2() -> String {
    r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#.to_string()
}

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

fn sorted_numbers(data: String) -> Vec<u32> {
    let mut nums: Vec<u32> = Vec::new();
    // source
    nums.push(0);
    // adapters
    for line in data.lines() {
        match line.parse::<u32>() {
            Ok(num ) => nums.push(num),
            Err(_) => {},
        }
    }
    nums.sort();
    // Link in the device
    nums.push(nums.last().unwrap() + 3);
    // println!("{:?}", nums);
    nums
}

fn is_valid(bubble: &[u32], skips: &Vec<usize>, base: &usize) -> bool {
    let mut prev = bubble[base - 1];
    for idx in *base..bubble.len() {
        if skips.contains(&idx) {
            continue;
        }
        let val = bubble[idx];
        if val - prev > 3 {
            return false;
        }
        prev = val;
    }
    true
}

fn count_combinations(bubble: &[u32]) -> u64 {
    // println!("{:?}", bubble);
    let mut combinations = 1;
    if bubble.len() <= 2 { return 1 }
    let mut skips: Vec<usize> = Vec::new();
    let mut idx = 1;
    let last_idx = bubble.len() - 1;
    skips.push(idx);
    loop {
        if is_valid(bubble, &skips, &skips.first().unwrap()) {
            combinations += 1;
            idx += 1;
        } else {
            idx = skips.pop().unwrap() + 1;
        }
        loop {
            if idx < last_idx {
                skips.push(idx);
                break;
            } else {
                if skips.is_empty() {
                    break;
                }
                idx = skips.pop().unwrap() + 1;
            }
        }
        if skips.is_empty() {
            break;
        }
    }
    combinations
}

fn scan_bubbles(nums: Vec<u32>) -> Vec<u64> {
    // println!("{:?}", nums);
    let mut combinations: Vec<u64> = Vec::new();
    let mut last_3_idx: usize = 0;
    let mut last_val: u32 = 0;
    for idx in 0..nums.len() {
        let val = nums[idx];
        if val - last_val == 3 {
            combinations.push(count_combinations(&nums[last_3_idx..idx]));
            last_3_idx = idx;
        }
        last_val = val;
    }
    combinations
}

fn main() {
    // let data = load_demo();
    // let data = load_demo2();
    let data = load_data();
    let nums = sorted_numbers(data);
    let mut steps: [usize; 4] = [0, 0, 0, 0];
    let mut jolts: u32 = 0;
    for num in nums.iter() {
        steps[(num - jolts) as usize] +=1;
        jolts = num.clone();
    }
    println!("Part 1: {:?} -> {}", steps, steps[1] * steps[3]);

    let combs = scan_bubbles(nums);
    let prod = combs.iter().fold(1, |a, b| a * b);
    println!("{:?} => {}", combs, prod);
}
