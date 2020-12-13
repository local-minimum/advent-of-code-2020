use std::{collections::HashMap, fs};

fn load_demo() -> String {
    r#"939
7,13,x,x,59,x,31,19"#.to_string()
}

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

#[derive(Debug)]
struct Schedule {
    interval: usize,
    index: usize,
}

impl Schedule {
    fn from_str(val: &str, index: usize) -> Option<Self> {
        match val {
            "x" => None,
            _ => {
                let interval: usize = val.parse().unwrap();
                Some(Schedule{interval, index})
            }
        }
    }

    fn departs_in(&self, now: &usize) -> usize {
        self.interval - now % self.interval
    }

    fn is_departure_sequance(&self, ref_time: &usize) -> bool {
        (ref_time + self.index) % self.interval == 0
    }
}


fn parse(data: &String) -> (usize, Vec<Schedule>) {
    let mut i = data.lines();
    let departure: usize = match i.next() {
        Some(v) => {
            v.trim().parse().unwrap()
        },
        None => panic!("No departure time"),
    };
    let mut schedules: Vec<Schedule> = Vec::new();
    let mut idx = 0;
    for raw_schedule in i.next().unwrap().split(",") {
        match Schedule::from_str(raw_schedule, idx) {
            None => {idx += 1},
            Some(schedule) => {
                schedules.push(schedule);
                idx += 1
            },
        }
    }
    (departure, schedules)
}

fn solve_part1(schedules: &Vec<Schedule>, departure: &usize) -> (usize, usize) {
    let mut wait: usize = 0;
    let mut bus_id: usize = 0;
    let mut any_bus: bool = false;
    for schedule in schedules {
        let bus_wait = schedule.departs_in(departure);
        if !any_bus {
            any_bus = true;
            wait = bus_wait;
            bus_id = schedule.interval;
        } else if bus_wait < wait {
            wait = bus_wait;
            bus_id = schedule.interval;
        }
    }
    (bus_id, departure + wait)
}

fn step_rule(schedules: &Vec<Schedule>) -> (usize, usize) {
    let mut interval = 0;
    let mut bus_index= 0;
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    for idx in 0..schedules.len() {
        let f = schedules[idx].interval;
        for idx2 in 0..schedules.len() {
            if idx == idx2 { continue; }
            if (schedules[idx2].index as i32 - schedules[idx].index as i32).abs() as usize % f == 0 {
                if map.contains_key(&schedules[idx2].index) {
                    map.get_mut(&schedules[idx2].index).unwrap().push(schedules[idx].interval);
                } else {
                    let v: Vec<usize> = vec![schedules[idx].interval, schedules[idx2].interval];
                    map.insert(schedules[idx2].index, v);
                }
            }
        }
        if f > interval {
            interval = f;
            bus_index = schedules[idx].index;
        } 
    }
    for (k, v) in map.iter() {
        let f = v.iter().fold(1, |a, b| a * b);
        if f > interval {
            interval = f;
            bus_index = *k;
        }
    }
    (bus_index, interval)
}

fn eval_time(schedules: &Vec<Schedule>, time: usize) -> bool {
    for schedule in schedules {
        if !schedule.is_departure_sequance(&time) {
            return false;
        }
    }
    true
}


fn solve_part2(schedules: &Vec<Schedule>) -> usize {
    let (offset, step_factor) = step_rule(schedules);
    println!("Step factor {} at offset {}", step_factor, offset);
    let mut factor = 1;
    loop {
        if eval_time(schedules, factor * step_factor - offset) {
            println!("Found solution after {} iterations", factor);
            return factor * step_factor - offset;
        } else if factor % 100 == 0 {
            println!("Failed on {}", factor * step_factor - offset);
        }
        factor += 1;
    }    
}

fn main() {
    // let data = load_demo();
    let data = load_data();
    let (now, schedules) = parse(&data);
    println!("{}, {:?}", now, schedules);
    let (bus, bus_departure) = solve_part1(&schedules, &now);
    let wait = bus_departure - now;
    println!("{} leaves at {} (wait for {})", bus, bus_departure, wait);
    println!("Part 1: {}", bus * wait);

    println!("{}", eval_time(&schedules, 1068781));
    let timestamp = solve_part2(&schedules);
    println!("Part 2: {}", timestamp);
}
