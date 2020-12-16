use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

fn load_demo() -> String {
    r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#.to_string()
}

#[derive(Debug)]
struct BagRule {
    color: String,
    quantity: usize,
}

fn parse_content(content: &str, cont_pattern: &Regex) -> Vec<BagRule> {
    if content == "no other bags." {
        return vec![];
    } else {
        let mut holds: Vec<BagRule> = vec![];
        for cap in cont_pattern.captures_iter(content) {
            let quantity: usize = cap[1].parse().unwrap();
            let color: String = cap[2].to_string();
            holds.push(BagRule {color, quantity});
        }
       return holds; 
    }
}

fn parse_rule(line: &str, pattern: &Regex, cont_pattern: &Regex) -> Option<(String, Vec<BagRule>)> {
    match pattern.captures(line.trim()) {
        Some(cap) => {
            let color = cap[1].to_string();
            let content = parse_content(&cap[2], cont_pattern);
            println!("{} -> {:?}", color, content);
            Some((color, content))
        },
        None => None,
    }
}

fn get_bags_that_can_contain(color: String, rev_rules: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut bags: Vec<String> = vec![];
    let mut candidates: Vec<String> = match rev_rules.get(&color) {
        Some(v) => v.clone(),
        None => vec![],
    };
    while candidates.len() > 0 {
        match candidates.pop() {
            Some(c) => {
                bags.push(c.clone());
                for b2 in get_bags_that_can_contain(c, rev_rules) {
                    if bags.iter().any(|v| v == &b2) || candidates.iter().any(|v| v == &b2) {
                        continue;
                    }
                    candidates.push(b2);
                }
            },
            None => break,
        }
    }
    bags
}

fn get_bags_in_bag(color: String, rules: &HashMap<String, Vec<BagRule>>) -> usize {    
    match rules.get(&color) {
        Some(content) => {
            let mut sum: usize = 0;
            for bag in content.iter() {
                sum += bag.quantity * (get_bags_in_bag(bag.color.clone(), rules) + 1);
            }
            sum
        },
        None => 0,
    }
}

fn main() {
    let re = Regex::new(r"^(\w+ \w+) bags contain (.*)$").unwrap();
    let cont_re = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    // let data = load_demo();
    let data = load_data();
    let mut rules: HashMap<String, Vec<BagRule>> = HashMap::new();
    let mut rev_rules: HashMap<String, Vec<String>> = HashMap::new();
    for line in data.lines() {
        match parse_rule(line, &re, &cont_re) {
            Some((color, content)) => {
                for bag in content.iter() {
                    match rev_rules.contains_key(&bag.color) {
                        true => {
                            rev_rules.get_mut(&bag.color).unwrap().push(color.clone());
                        },
                        false => {
                            rev_rules.insert(bag.color.clone(), vec![color.clone(); 1]);
                        }
                    }
                }
                rules.insert(color, content);                
            },
            None => {},
        }
    }

    let bags = get_bags_that_can_contain("shiny gold".to_string(), &rev_rules);
    println!("\nPart 1: {} bags: {:?}", bags.len(), bags);
    let nbags = get_bags_in_bag("shiny gold".to_string(), &rules);
    println!("\nPart 2: {} bags in a shiny gold bag", nbags);
}
