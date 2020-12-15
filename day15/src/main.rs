use std::collections::HashMap;

fn load_data(variant: usize) -> String {
    match variant {
        0 => "0,3,6",
        1 => "1,3,2",
        2 => "2,1,3",
        3 => "1,2,3",
        4 => "2,3,1",
        5 => "3,2,1",
        6 => "3,1,2",
        7 => "7,14,0,17,11,1,2",
        _ => panic!("Variant {} not known", variant)
    }.to_string()
}

fn to_numbers(data: &String) -> Vec<usize> {
    data
        .split(",")
        .map(|v| v.trim().parse::<usize>().unwrap())
        .collect()
}

fn rules(map: &HashMap<usize, usize>, last: &usize, len: &usize) -> usize {
    match map.get(last) {
        Some(val) => len - (val + 1),
        None => 0,
    }
}

fn to_hasmap(seq: Vec<usize>) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    for idx in 0..seq.len() - 1  {
        map.insert(seq[idx].clone(), idx);
    }
    map
}

fn main() {
    // let turns = 10;
    // let turns = 2020;
    let turns = 30000000;
    for variant in 0..8 {
        let data = load_data(variant);
        //print!("{},", data);
        let sequence = to_numbers(&data);
        let mut len = sequence.len();
        let mut last: usize = sequence.last().unwrap().clone();
        let mut map = to_hasmap(sequence);
        let mut same: bool = false;
        while len < turns {
            let next = match same {
                true => 1,
                false => rules(&map, &last, &len),
            };
            //println!("{}=>{} ({}) {:?}", last, next, len, map);
            map.insert(last, len - 1);
            len += 1;
            same = last == next;
            last = next;
            //print!("{}{}", match same { true => 'T', false => 'F'},next);
        }
        println!("Solution: {}", last);
    }
}
