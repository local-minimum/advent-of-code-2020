extern crate pancurses;
use std::{collections::{HashMap, HashSet}, fs, ops::{Add, AddAssign}};
use pancurses::{Input, Window, endwin, initscr, noecho};

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

    fn to_xy(&self, ref_x: &i32, ref_y: &i32) -> Option<(i32, i32)> {
        let x = ref_x + self.q * 2 + self.r;
        let y = ref_y + self.r;

        if x > 0 && y > 0 {
            Some((x, y))
        } else {
            None
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

fn next_day(mut blacks: Vec<CubeCoords>) -> Vec<CubeCoords> {
    let mut neighbours: HashMap<AxialCoords, usize> = HashMap::new();
    for black in blacks.iter() {
        for n in black.neighbours() {
            let count = neighbours.entry(AxialCoords::from_cube(&n)).or_insert(0);
            *count += 1;
        }
    }
    let old_black: HashSet<AxialCoords> = blacks
        .iter()
        .map(|c| AxialCoords::from_cube(c))
        .collect();

    let new_black: Vec<CubeCoords> = neighbours
        .iter()
        .filter(|(c, v)| **v == 2 && !old_black.contains(c))
        .map(|(c, _)| CubeCoords::from_axial(c))
        .collect();

    blacks = blacks
        .iter()
        .map(|c| (c, *neighbours.get(&AxialCoords::from_cube(c)).or(Some(&0)).unwrap()))
        .filter(|(_, n)| *n == 1 || *n == 2)
        .map(|(c, _)| c)
        .copied()
        .collect();
    blacks.extend(new_black);
    blacks
}

fn draw(window: &Window, part1: &usize, day: &usize, blacks: &Vec<CubeCoords>) {
    window.clear();
    for b in blacks {
        match AxialCoords::from_cube(b).to_xy(&40, &11) {
            Some((x, y)) => {
                window.mvaddstr(y, x - 1, "[]");
            },
            None => {},
        }
    }
        
    window.mvaddstr( 23, 1, format!("Part 1: #{} black | Day {}, {} black", part1, day, blacks.len()));
    window.mvaddstr( 23, 60, "(n)ext day / (q)uit ");
    window.keypad(true);
    window.border('|', '|', '-', '-', '/', '\\', '\\', '/');
    window.refresh();
}

fn main() {
    let is_demo = false;
    let window = initscr();
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
    let part1 = blacks.len();
    let mut day = 0;
    noecho();
    draw(&window, &part1, &day, &blacks);
    loop {
        match window.getch() {
            Some(Input::Character('q')) => break,
            Some(Input::Character('n')) => {
                blacks = next_day(blacks);
                day += 1;
                draw(&window, &part1, &day, &blacks);
            }
            _ => {},
        }
    }
    endwin();
}
