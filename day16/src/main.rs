use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn load_demo() -> String {
    r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#.to_string()
}

fn load_demo2() -> String {
    r#"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"#.to_string()
}

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

#[derive(Debug)]
struct Rule {
    name: String,
    lb_a: usize,
    ub_a: usize,
    lb_b: usize,
    ub_b: usize,
}

impl Rule {
    fn from_string(line: &str) -> Option<Self> {
        let re = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        match re.captures(line) {
            Some(caps) => {
                let lb_a: usize = caps[2].parse::<usize>().unwrap();
                let ub_a: usize = caps[3].parse::<usize>().unwrap();
                let lb_b: usize = caps[4].parse::<usize>().unwrap();
                let ub_b: usize = caps[5].parse::<usize>().unwrap();
                Some(Rule{
                    name: caps[1].to_string(),
                    lb_a,
                    ub_a,
                    lb_b,
                    ub_b,
                })
            },
            None => None,
        }
    }

    fn matches(&self, value: &usize) -> bool {
        *value >= self.lb_a && *value <= self.ub_a || *value >= self.lb_b && *value <= self.ub_b
    }
}

fn to_numbers(line: &str) -> Vec<usize> {
    line
        .split(",")
        .map(|v| v.trim().parse().unwrap())
        .collect()
}
#[derive(Debug)]
enum ParsePhase {Rules, OwnHeader, Own, NearbyHeader, Nearby}

fn parse(data: String) -> (
    Vec<Rule>,
    Vec<usize>,
    Vec<Vec<usize>>,
) {
    let mut phase = ParsePhase::Rules;
    let mut rules: Vec<Rule> = vec![];
    let mut own: Vec<usize> = vec![];
    let mut nearby: Vec<Vec<usize>> = vec![];
    for line in data.lines() {
        match phase {
            ParsePhase::Rules => {
                phase = match Rule::from_string(line) {
                    Some(rule) => {
                        rules.push(rule);
                        ParsePhase::Rules
                    },
                    None => ParsePhase::OwnHeader,
                };
            },
            ParsePhase::OwnHeader => {
                if !line.starts_with("your ticket:") {
                    panic!("Unexpected line ({:?}): {}", phase, line);
                }
                phase = ParsePhase::Own;
            }
            ParsePhase::Own => {
                if own.len() == 0 {
                    own.append(&mut to_numbers(line));
                } else if line.len() > 0 {
                    panic!("Unexpected line ({:?}): {}", phase, line);
                } else {
                    phase = ParsePhase::NearbyHeader;
                }
            }
            ParsePhase::NearbyHeader => {
                if !line.starts_with("nearby tickets:") {
                    panic!("Unexpected line ({:?}): {}", phase, line);
                }
                phase = ParsePhase::Nearby                
            }    
            ParsePhase::Nearby => {
                nearby.push(to_numbers(line));
            }
        }
    }
    // println!("{:?}", rules);
    // println!("My {:?}", own);
    // println!("Nearby:\n{:?}", nearby);
    (rules, own, nearby)
}

fn invalid_numbers(
    rules: &Vec<Rule>,
    ticket: &Vec<usize>,
) -> Vec<usize> {
    ticket
        .iter()
        .filter(| v | !rules.iter().any(| r| r.matches(v)))
        .map(|v| v.clone())
        .collect()
}

fn init_mapping(rules: &Vec<Rule>) -> HashMap<usize, Vec<usize>> {
    let l = rules.len();
    let mut mapping = HashMap::new();
    for idx in 0..l {
        mapping.insert(idx, (0..l).collect());
    }
    mapping
}

fn update_mapping(
    ticket: &Vec<usize>,
    mapping: &mut HashMap<usize, Vec<usize>>,
    rules: &Vec<Rule>
) {
    //println!("{:?}", mapping);
    for id_pos in 0..ticket.len() {
        for id_rule in (0..mapping[&id_pos].len()).rev() {
            // println!("{}->{} ({} in {:?} {})", id_pos, id_rule, near[id_pos], rules[id_rule], rules[id_rule].matches(&near[id_pos]));
            if !rules[mapping[&id_pos][id_rule]].matches(&ticket[id_pos]) {
                mapping.get_mut(&id_pos).unwrap().remove(id_rule);
            }
        }
    }
    //print_ticket(&ticket, &mapping, &rules);
    while cleanup_mapping(mapping) {}
}

fn cleanup_mapping(
    mapping: &mut HashMap<usize, Vec<usize>>,
) -> bool {
    let mut changed = false;
    for id_pos in 0..mapping.len() {
        if mapping[&id_pos].len() == 1 {
            let val = mapping[&id_pos][0];
            for id_pos2 in 0..mapping.len() {
                if id_pos == id_pos2 {
                    continue;
                }

                let v = mapping.get_mut(&id_pos2).unwrap();
                let before = v.len();
                v.retain(|v| v != &val);
                if !changed && before != v.len() {
                    changed = true;
                }
            }
        } 
    }
    changed
}

fn print_ticket(
    ticket: &Vec<usize>,
    mapping: &HashMap<usize, Vec<usize>>,
    rules: &Vec<Rule>
) {
    for id_pos in 0..ticket.len() {
        for id_rule in mapping.get(&id_pos).unwrap() {
            let rule = rules.get(*id_rule).unwrap();
            print!("{}\t", rule.name);
        }
        println!("{}", ticket[id_pos]);
    }
    println!("");
}

fn main() {
    // let data = load_demo();
    // let data = load_demo2();
    let data = load_data();
    let (rules, own, nearby) = parse(data);
    let mut invalids: Vec<usize> = vec![];
    let mut valids: Vec<Vec<usize>> = vec![];
    for near in nearby {
        let mut invalid_nums = invalid_numbers(&rules, &near);
        if invalid_nums.is_empty() {
            valids.push(near);
        } else {
            invalids.append(&mut invalid_nums);
        }
    }
    let error_rate = invalids.iter().fold(0, |a, b| a + *b);
    println!("Part 1: {}", error_rate);
    // println!("Passing {:?}", valids);
    let mut mapping = init_mapping(&rules);
    update_mapping(&own, &mut mapping, &rules);
    for near in valids {
        update_mapping(&near, &mut mapping, &rules);
    }
    // print_ticket(&own, &mapping, &rules);
    let mut prod = 1;
    for (id_pos, id_rules) in mapping {
        if id_rules.len() != 1 { panic!("Ambiguity!")}
        let id_rule = id_rules.first().unwrap();
        if rules[*id_rule].name.starts_with("departure") {
            prod *= own[id_pos];
        }
    }
    println!("Part 2: {}", prod);
}

