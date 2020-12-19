use std::collections::{HashMap, HashSet};
use std::fs;

fn load_demo1() -> String {
    r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"

a
b
ab
ba
aab
aba
aa
bb
bba
aaa
bbb"#.to_string()
}

fn load_demo2() -> String {
 r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

aaba
bbba
aaabab
aabaab
ababbb"#.to_string()   
}

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

#[derive(Debug)]
struct Rule {
    ch: Option<char>,
    decendants: Vec<usize>,
    optional: Vec<usize>,
}

#[derive(PartialEq)]
#[allow(dead_code)]
enum Verbosity {Minimal, Low, Medium, High}

impl Verbosity {
    fn level0(&self) -> bool {
        match &self {
            Verbosity::Minimal  => false,
            _ => true,
        }
    }

    fn level1(&self) -> bool {
        match &self {
            Verbosity::High | Verbosity::Medium => true,
            _ => false,
        }
    }

    fn level2(&self) -> bool {
        match &self {
            Verbosity::High => true,
            _ => false,
        }
    }
}

impl Rule {

    fn _parse_part(part: &str) -> Vec<usize> {
        part
            .trim()
            .split(" ")
            .map(|v| v.parse().unwrap())
            .collect()
    }

    fn from_instructions(rule: &str) -> Self {
        if rule.contains("\"") {
            let chs: Vec<char>  = rule.chars().collect();
            let ch: char = chs[1];
            Rule{ch: Some(ch), decendants: vec![], optional: vec![]}
        } else if rule.contains("|") {
            let parts: Vec<&str> = rule.split("|").collect();
            let dec = Rule::_parse_part(parts[0]);
            let opt = Rule::_parse_part(parts[1]);
            Rule{ch: None, decendants: dec, optional: opt}
        } else {
            let dec = Rule::_parse_part(rule);
            Rule{ch: None, decendants: dec, optional: vec![]}
        }
    }

    fn matches(&self, line: &Vec<char>, pos: &usize, rules: &HashMap<usize, Rule>, verbose: &Verbosity) -> Vec<usize> {
        let mut rec: Vec<usize> = vec![];
        match self.ch {
            Some(ch) => {
                if *pos >= line.len() {
                    if verbose.level2() { println!("Out of line {}", pos); }
                    return vec![]
                } else if line[*pos] == ch {
                    if verbose.level2() { println!("Matches {} @ {}", ch, pos);}
                    rec.push(pos + 1);
                } else {
                    if verbose.level2() { println!("Fails {} @ {}", ch, pos);}
                    return vec![];
                }
            },
            None => {
                let mut v: HashSet<usize> = HashSet::new();
                v.insert(pos.clone());                
                for (depth, dec) in self.decendants.iter().enumerate() {
                    let mut next_v: HashSet<usize> = HashSet::new();
                    if verbose.level2() { println!("{}: Testing rule {}", depth, dec);}
                    for val in v {
                        let results = rules[dec].matches(line, &val, rules, verbose);
                        if verbose.level2() { println!("\t{}: Results rule {} @ {} ({:?})", depth, dec, val, results);}
                        next_v.extend(results);
                    }
                    if verbose.level2() { println!("\t\t{:?}", next_v);}
                    v = next_v;
                }
                if v.contains(&(line.len() - 1)) {
                    return vec![line.len() - 1];
                }
                for val in v {
                    if val == *pos { continue; }
                    rec.push(val);
                }
                if self.optional.is_empty() {
                    return rec;
                }
                v = HashSet::new();
                v.insert(pos.clone());
                for (depth, dec) in self.optional.iter().enumerate() {
                    if verbose.level2() { println!("{}: Testing rule {}", depth, dec);}
                    let mut next_v: HashSet<usize> = HashSet::new();
                    for val in v {
                        let results = rules[dec].matches(line, &val, rules, verbose);
                        if verbose.level2() { println!("\t{}: Results rule {} @ {} ({:?})", depth, dec, val, results);}
                        next_v.extend(results);
                    }
                    if verbose.level2() { println!("\t\t{:?}", next_v);}
                    v = next_v;
                }
                if v.contains(&(line.len() - 1)) {
                    return vec![line.len() - 1];
                }
                for val in v {
                    if val == *pos { continue; }
                    rec.push(val);
                } 
            }
        }
        rec
    }

    /* Slow initial solution
    fn patterns(&self, rules: &HashMap<usize, Rule>, verbose: &Verbosity) -> Vec<String> {
        let mut rec: Vec<String> = vec![];
        match self.ch {
            Some(ch) => {
                rec.push(format!("{}", ch));
            },
            None => {
                let mut v: Vec<String> = vec!["".to_string()];
                for dec in self.decendants.iter() {
                    let mut next_v: Vec<String> = vec![];
                    for val in v {
                        for r in rules[dec].patterns(rules, verbose) {
                            next_v.push(format!("{}{}", val, r));
                       }
                    }
                    v = next_v;
                }
                for val in v {
                    if val.len() == 0 { continue; }
                    rec.push(val);
                } 
                v = vec!["".to_string()];
                for dec in self.optional.iter() {
                    let mut next_v: Vec<String> = vec![];
                    for val in v {
                        for r in rules[dec].patterns(rules, verbose) {
                            next_v.push(format!("{}{}", val, r));
                       }
                    }
                    v = next_v;
                } 
                for val in v {
                    if val.len() == 0 { continue; }
                    rec.push(val);
                } 
            }
        }
        if verbose.level2() {println!("{:?} => {:?}", self, rec); }
        rec
    }
    */
}

#[derive(Debug)]
struct RuleSet {
    rules: HashMap<usize, Rule>,
}

impl RuleSet {
    fn from_string(data: &String) -> (Self, String) {
        let mut lines = data.lines();
        let mut rules: HashMap<usize, Rule> = HashMap::new();
        while let Some(line) = lines.next() {
            if line == "" { break; }            
            let parts: Vec<&str> = line.split(":").collect();
            let num: usize = parts[0].parse().unwrap();
            let rule = parts[1].trim();
            rules.insert(num, Rule::from_instructions(rule));
        }
        (RuleSet{rules}, lines.map(|v| format!("{}\n", v)).collect())
    }

    /* Slow initial solution
    fn matches(&self, line: &str, rule_zero: &bool, verbose: &Verbosity) -> bool {
        for (rule_id, rule) in self.rules.iter() {
            if *rule_zero && *rule_id != 0 { continue; }
            if verbose.level1() { println!("\n ---- {} ----", rule_id);}
            for pattern in rule.patterns(&self.rules, verbose) {
                if pattern == line {
                    if verbose.level1() { println!("YES!"); }
                    return true;
                }
                if verbose.level2() { println!("NO! {} != {}", line, pattern); }
            }
        }
        false
    }
    */

    fn apply_part_two(&mut self) {
        self.rules.insert(8, Rule::from_instructions("42 | 42 8"));
        self.rules.insert(11, Rule::from_instructions("42 31 | 42 11 31"));
    }
}

fn main() {
    let verbose = Verbosity::Low;
    let is_demo = false;
    let is_second_demo = false;
    let is_part_two = true;
    let data = match (is_demo, is_second_demo) {
        (true, false) => load_demo1(),
        (true, true) => load_demo2(),
        _ => load_data(),
    };
    let (mut rules, messages) = RuleSet::from_string(&data);
    if is_part_two { rules.apply_part_two() }
    let rule0 = rules.rules.get(&0).unwrap();
    let mut matching = 0;
    for line in messages.lines() {
        let chars: Vec<char> = line.chars().collect();
        let positions = rule0.matches(&chars, &0, &rules.rules, &verbose);
        let val = positions.contains(&(chars.len()));
        if verbose.level1() { println!("{:?}", positions); }
        if verbose.level0() {
            if val {
                println!(" OK: {}", line);
            } else {
                println!("NOK: {}", line);
            }
        }
        if val {
            matching +=1;
        }
    }
    /* This is very slow and only works for Part 1   
    let patterns = rules
        .rules
        .get(&0)
        .unwrap()
        .patterns(&rules.rules, &verbose);

    let mut hashed: HashSet<String> = HashSet::new();
    for p in patterns {
        hashed.insert(p);
    }
    for line in messages.lines() {
        let val = hashed.contains(&line.to_string());
        if verbose.level0() {
            if val {
                println!(" OK: {}", line);
            } else {
                println!("NOK: {}", line);
            }
        }
        if val {
            matching +=1;
        }
    }
    */
    println!("Matching messages {}", matching);
}
