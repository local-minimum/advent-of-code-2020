use std::fs;

fn load_demo() -> String {
    r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#.to_string()
}

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

fn to_2d(data: &String) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        grid.push(line.chars().collect());
    }

    grid
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        let mut s: String = String::from("");
        for val in row {
            s.push(val.clone());
        }
        println!("{}", s);
    }
}

fn count_neighbours(grid: &Vec<Vec<char>>, rid: &usize, cid: &usize) -> usize {
    let mut neighbours = 0;
    for roff in -1..2 {
        if *rid as i32 + roff < 0 {
            continue;
        }
        for coff in -1..2 {
            if roff == 0 && coff == 0 || *cid as i32 + coff < 0 {
                continue;
            }
            let occ = match grid.get((*rid as i32 + roff) as usize) {
                Some(row) => {
                    match row.get((*cid as i32 + coff) as usize) {
                        Some(val) => *val == '#',
                        None => false,
                    }
                },
                None => false,
            };
            if occ {
                neighbours += 1;
            }
        }
    }
    // println!("{} {} => {}", rid, cid, neighbours);
    neighbours
}

fn rule_book(grid: &Vec<Vec<char>>, rid: &usize, cid: &usize) -> char {
    match grid[*rid][*cid] {
        'L' => match count_neighbours(grid, rid, cid) {
            0 => '#',
            _ => 'L',
        },
        '#' => match count_neighbours(grid, rid, cid) {
            0..=3 => '#',
            _ => 'L',
        },
        _ => grid[*rid][*cid].clone(),
    }
}

fn count_neighbours2(grid: &Vec<Vec<char>>, rid: i32, cid: i32) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut neighbours = 0;
    for roff in -1..2 {
        for coff in -1..2 {
            if roff == 0 && coff == 0 {
                continue;
            }
            let mut r = rid;
            let mut c = cid;
            loop {
                r += roff;
                c += coff;
                if r < 0 || c < 0 || c == cols || r == rows {
                    break;
                }
                if match grid[r as usize][c as usize] {
                    'L' => true,
                    '#' => {
                        neighbours += 1;
                        true
                    },
                    _ => false,
                } {
                    break;
                }
            }
        }
    }
    neighbours
}

fn rule_book2(grid: &Vec<Vec<char>>, rid: &usize, cid: &usize) -> char {
    match grid[*rid][*cid] {
        'L' => match count_neighbours2(grid, *rid as i32, *cid as i32) {
            0 => '#',
            _ => 'L',
        },
        '#' => match count_neighbours2(grid, *rid as i32, *cid as i32) {
            0..=4 => '#',
            _ => 'L',
        },
        _ => grid[*rid][*cid].clone(),
    }
}

fn mutate_grid(
    grid: &Vec<Vec<char>>,
    rule: fn(grid: &Vec<Vec<char>>, rid: &usize, cid: &usize) -> char,
) -> Vec<Vec<char>> {
    let mut mutant: Vec<Vec<char>> = Vec::new();
    for rid in 0..grid.len() {
        let mut mutant_row: Vec<char> = Vec::new();
        for cid in 0..grid[rid].len() {
            mutant_row.push(rule(grid, &rid, &cid));
        }
        mutant.push(mutant_row);
    }
    mutant
}

fn count_occupied(grid: &Vec<Vec<char>>) -> usize {
    let mut occ = 0;
    for row in grid {
        occ += row.iter().filter(|c| **c == '#').count();
    }
    occ
}

fn simulate(
    data: &String,
    verbose: &bool,
    rule: fn(grid: &Vec<Vec<char>>, rid: &usize, cid: &usize) -> char,
) -> (usize, usize) {
    let mut iterations: usize = 0;
    let mut prev = to_2d(data);
    if *verbose {
        print_grid(&prev);
    }
    loop {
        let grid = mutate_grid(&prev, rule);
        iterations += 1;
        if *verbose {
            println!("");
            print_grid(&grid);
        }
        if grid == prev {
            break;
        }
        prev = grid;
    }
    (iterations, count_occupied(&prev))
}

fn main() {
    // let data = load_demo();
    let data = load_data();
    let verbose = false;
    let (iterations, occupied) = simulate(&data, &verbose, rule_book);
    println!("\nFound stable solution after {} iterations", iterations);
    println!("Step 1: {} occupied", occupied);
    let (iterations2, occupied2) = simulate(&data, &verbose, rule_book2);
    println!("\nFound stable solution after {} iterations", iterations2);
    println!("Step 2: {} occupied", occupied2);
}
