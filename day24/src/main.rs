use std::{collections::{HashMap}, ops::{Add, AddAssign}, fs};

fn load_demo() -> String {
    r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#.to_string()
}

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct CubeCoords {
    x: i32,
    y: i32,
    z: i32,
}

impl AddAssign for CubeCoords {
    
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Add for CubeCoords {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
       CubeCoords {
           x: self.x + other.x,
           y: self.y + other.y,
           z: self.z + other.z,
       } 
    }
}

impl CubeCoords {
    fn from_str(direction: &str) -> CubeCoords {
        match direction {
            "e" => CubeCoords{x: 1, y: -1, z: 0},
            "se" => CubeCoords{x: 0, y: -1, z: 1},
            "sw" => CubeCoords{x: -1, y: 0, z: 1},            
            "w" => CubeCoords{x: -1, y: 1, z: 0},
            "nw" => CubeCoords{x: 0, y: 1, z: -1},
            "ne" => CubeCoords{x: 1, y: 0, z: -1},
            _ => panic!("{} is not a direction", direction),
        }
    }

    fn from_axial(axial: &AxialCoords) -> Self {
        Self {
            x: axial.q,
            y: -axial.q - axial.r, 
            z: axial.r,            
        }
    }

    fn neighbours(&self) -> Vec<CubeCoords> {
        vec![
            *self + CubeCoords{x: 1, y: -1, z: 0},
            *self + CubeCoords{x: 0, y: -1, z: 1},
            *self + CubeCoords{x: -1, y: 0, z: 1},            
            *self + CubeCoords{x: -1, y: 1, z: 0},
            *self + CubeCoords{x: 0, y: 1, z: -1},
            *self + CubeCoords{x: 1, y: 0, z: -1},
        ]
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct AxialCoords {
    q: i32,
    r: i32,
}

impl AxialCoords {
    fn from_cube(cube: &CubeCoords) -> Self {
        Self{
            q: cube.x,
            r: cube.z,
        }
    }
}

fn parse_line(line: &str) -> CubeCoords {
    let mut coords = CubeCoords{x: 0, y: 0, z: 0};
    let mut register: Option<char> = None;
    for ch in line.chars() {
        match ch {
            's' | 'n' => register = Some(ch),
            'w' | 'e' => {
                match register {
                    Some(reg) => coords += CubeCoords::from_str(&format!("{}{}", reg, ch)),
                    None => coords += CubeCoords::from_str(&format!("{}", ch)),
                }
                register = None;
            },
            _ => panic!("{} has unhandled character {}", line, ch),
        }
    }
    coords
}


fn main() {
    let is_demo = true;
    let data = match is_demo { true => load_demo(), false => load_data() };
    let mut tiles = vec![];
    for line in data.lines() {
        tiles.push(parse_line(line));
    }
    let mut flips: HashMap<AxialCoords, usize> = HashMap::new();
    for tile in tiles.iter() {
        let count = flips.entry(AxialCoords::from_cube(tile)).or_insert(0);
        *count += 1;
    }
    let mut blacks: Vec<CubeCoords> = flips
        .iter()
        .filter(|(_, v)| *v % 2 == 1)
        .map(|(ax, _)| CubeCoords::from_axial(ax))
        .collect();
    println!("{} black tiles {} manipulated", blacks.len(), flips.len());
    let mut neighbours: HashMap<AxialCoords, usize> = HashMap::new();
    for black in blacks.iter() {
        for n in black.neighbours() {
            let count = neighbours.entry(AxialCoords::from_cube(&n)).or_insert(0);
            *count += 1;
        }
    }
    blacks = blacks
        .iter()
        .filter(|c| *neighbours.get(&AxialCoords::from_cube(c)).or(Some(&0)).unwrap() != 2)
        .copied()
        .collect();

    println!("Day 1 (remove rule 1): {}", blacks.len());
    blacks.extend(
        neighbours
            .iter()
            .filter(|(_, v)| **v == 2)
            .map(|(c, _)| CubeCoords::from_axial(c))
    );
    println!("Day 1: {}", blacks.len());
}
