use std::fs;
use std::ops::AddAssign;
use regex::Regex;

fn read_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

#[derive(Debug)]
struct Pass {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: usize,
    hcl: usize,
    ecl: usize,
    pid: usize,
    cid: usize,
}

impl Pass {
    fn is_passport(&self) -> bool {
        self.byr == 1 &&
        self.iyr == 1 &&
        self.eyr == 1 && 
        self.hgt == 1 &&
        self.hcl == 1 &&
        self.ecl == 1 &&
        self.pid == 1 &&
        self.cid == 1
    }

    fn is_northpole_creds(&self) -> bool {
        self.byr == 1 &&
        self.iyr == 1 &&
        self.eyr == 1 && 
        self.hgt == 1 &&
        self.hcl == 1 &&
        self.ecl == 1 &&
        self.pid == 1 &&
        self.cid == 0
    }
}

impl AddAssign for Pass {
    fn add_assign(&mut self, other: Self) {
        self.byr += other.byr;
        self.iyr += other.iyr;
        self.eyr += other.eyr;
        self.hgt += other.hgt;
        self.hcl += other.hcl;
        self.ecl += other.ecl;
        self.pid += other.pid;
        self.cid += other.cid;
    }
}

fn field_index(data: &Vec<&str>, pat: &str) -> Option<usize> {
    let mut idx = 0;
    for v in data {
        if v.starts_with(pat) {
            return Some(idx);
        }
        idx += 1;
    }
    return None
}

fn field_value_numeric(field: &str) -> Option<usize> {
    let (_, num) = field.split_at(4);
    match num.parse::<usize>() {
        Ok(val) => Some(val),
        Err(_) => None
    }
}

fn is_field(val: &Option<usize>) -> bool {
    match val {
        Some(_) => true,
        None => false
    }
}

fn validate_numeric_range(
    idx: Option<usize>,
    data: &Vec<&str>,
    low: usize,
    high: usize,
) -> usize {
    match idx {
        None => 0,
        Some(i) => {
            match field_value_numeric(data[i]) {
                None => 0,
                Some(val) => {
                    if low <= val && val <= high {
                        return 1
                    }
                    0
                },
            }
        }
    }
}

fn validate_height(
    idx: Option<usize>,
    data: &Vec<&str>,
) -> usize {
    match idx {
        Some(i) => {
            let field = data[i];
            if field.ends_with("cm") {
                match field_value_numeric(field.strip_suffix("cm").unwrap()) {
                    Some(val) => {
                        if val >= 150 && val <= 193 {
                            return 1
                        }
                        0
                    },
                    None => 0,
                }
            } else if field.ends_with("in") {
                match field_value_numeric(field.strip_suffix("in").unwrap()) {
                    Some(val) => {
                        if val >= 59 && val <= 76 {
                            return 1
                        }
                        0
                    },
                    None => 0,
                }
            } else {
                return 0
            }
            
        }
        None => { return 0 }
    }
}

fn validate_regex(
    idx: Option<usize>,
    data: &Vec<&str>,
    re: &Regex,
) -> usize {
    match idx {
        None => 0,
        Some(i) => {
             re.is_match(data[i]) as usize
        }
    }
}

fn validate_ecl(
    idx: Option<usize>,
    data: &Vec<&str>,
    hcls: &[&str; 7],
) -> usize {
    match idx {
        None => 0,
        Some(i) => {
            let (_, ln) = data[i].split_at(4);
            hcls.iter().any(|v| &ln == v ) as usize
        }
    }
}

fn create_pass(
    data: Vec<&str>,
    re_hcl: &Regex,
    re_pid: &Regex,
    hcls: &[&str; 7],
) -> (Pass, Pass) {
    let byr = field_index(&data, "byr");
    let iyr = field_index(&data, "iyr");
    let eyr = field_index(&data, "eyr");
    let hgt = field_index(&data, "hgt");
    let hcl = field_index(&data, "hcl");
    let ecl = field_index(&data, "ecl");
    let pid = field_index(&data, "pid");
    let cid = field_index(&data, "cid");
    (Pass {
        byr: is_field(&byr) as usize,
        iyr: is_field(&iyr) as usize,
        eyr: is_field(&eyr) as usize,
        hgt: is_field(&hgt) as usize,
        hcl: is_field(&hcl) as usize,
        ecl: is_field(&ecl) as usize,
        pid: is_field(&pid) as usize,
        cid: is_field(&cid) as usize,
    }, Pass {
        byr: validate_numeric_range(byr, &data, 1920, 2002),
        iyr: validate_numeric_range(iyr, &data, 2010, 2020),
        eyr: validate_numeric_range(eyr, &data, 2020, 2030),
        hgt: validate_height(hgt, &data),
        hcl: validate_regex(hcl, &data, re_hcl),
        ecl: validate_ecl(ecl, &data, hcls),
        pid: validate_regex(pid, &data, re_pid),
        cid: is_field(&cid) as usize,
    })
}

fn load_demo() -> String {
    let lit = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
    "#;
    lit.to_string()
}


fn main() {
    let mut totals = Pass{ byr: 0, ecl: 0, hcl: 0, hgt: 0, pid: 0, cid: 0, iyr: 0, eyr: 0};
    let re_hcl = Regex::new(r"^hcl:#[0-9a-f]{6}$").unwrap();
    let re_pid = Regex::new(r"^pid:\d{9}$").unwrap();
    let ecls: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let mut passports = 0;
    let mut northpoles = 0;
    let mut passports_strict = 0;
    let mut northpoles_strict = 0;
    // let data = load_demo();
    let data = read_data();
    for pass in data.split("\n\n") {
        let one_line = pass.replace("\n", " ");
        let words: Vec<&str> = one_line 
            .split(" ")
            .map(|w| w.trim())
            .collect();
        let (pass, strict_pass) = create_pass(words, &re_hcl, &re_pid, &ecls);
        if pass.is_passport() { passports +=1; }
        if pass.is_northpole_creds() { northpoles += 1; }
        if strict_pass.is_passport() { passports_strict += 1}
        if strict_pass.is_northpole_creds() { northpoles_strict += 1}
        totals += strict_pass;
    }

    println!("Part 1, Pass: {} North Poles: {} Sum: {}", passports, northpoles, passports + northpoles);
    println!("Part 2, Pass: {} North Poles: {} Sum: {}", passports_strict, northpoles_strict, passports_strict + northpoles_strict);
    println!("Debug individual strict rule passes {:?}", totals);
}
