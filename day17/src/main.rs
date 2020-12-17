use std::collections::HashSet;


fn load_demo() -> String {
    r#".#.
..#
###"#.to_string()
}

fn load_data() -> String {
    r#"#....#.#
..##.##.
#..#..#.
.#..#..#
.#..#...
##.#####
#..#..#.
##.##..#"#.to_string()
}

fn load_expected_4d() -> String {
    r#"Before any cycles:

z=0, w=0
.#.
..#
###


After 1 cycle:

z=-1, w=-1
#..
..#
.#.

z=0, w=-1
#..
..#
.#.

z=1, w=-1
#..
..#
.#.

z=-1, w=0
#..
..#
.#.

z=0, w=0
#.#
.##
.#.

z=1, w=0
#..
..#
.#.

z=-1, w=1
#..
..#
.#.

z=0, w=1
#..
..#
.#.

z=1, w=1
#..
..#
.#.


After 2 cycles:

z=-2, w=-2
.....
.....
..#..
.....
.....

z=-1, w=-2
.....
.....
.....
.....
.....

z=0, w=-2
###..
##.##
#...#
.#..#
.###.

z=1, w=-2
.....
.....
.....
.....
.....

z=2, w=-2
.....
.....
..#..
.....
.....

z=-2, w=-1
.....
.....
.....
.....
.....

z=-1, w=-1
.....
.....
.....
.....
.....

z=0, w=-1
.....
.....
.....
.....
.....

z=1, w=-1
.....
.....
.....
.....
.....

z=2, w=-1
.....
.....
.....
.....
.....

z=-2, w=0
###..
##.##
#...#
.#..#
.###.

z=-1, w=0
.....
.....
.....
.....
.....

z=0, w=0
.....
.....
.....
.....
.....

z=1, w=0
.....
.....
.....
.....
.....

z=2, w=0
###..
##.##
#...#
.#..#
.###.

z=-2, w=1
.....
.....
.....
.....
.....

z=-1, w=1
.....
.....
.....
.....
.....

z=0, w=1
.....
.....
.....
.....
.....

z=1, w=1
.....
.....
.....
.....
.....

z=2, w=1
.....
.....
.....
.....
.....

z=-2, w=2
.....
.....
..#..
.....
.....

z=-1, w=2
.....
.....
.....
.....
.....

z=0, w=2
###..
##.##
#...#
.#..#
.###.

z=1, w=2
.....
.....
.....
.....
.....

z=2, w=2
.....
.....
..#..
.....
....."#.to_string()
}

fn load_expected() -> String {
    r#"Before any cycles:

z=0
.#.
..#
###


After 1 cycle:

z=-1
#..
..#
.#.

z=0
#.#
.##
.#.

z=1
#..
..#
.#.


After 2 cycles:

z=-2
.....
.....
..#..
.....
.....

z=-1
..#..
.#..#
....#
.#...
.....

z=0
##...
##...
#....
....#
.###.

z=1
..#..
.#..#
....#
.#...
.....

z=2
.....
.....
..#..
.....
.....


After 3 cycles:

z=-2
.......
.......
..##...
..###..
.......
.......
.......

z=-1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=0
...#...
.......
#......
.......
.....##
.##.#..
...#...

z=1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=2
.......
.......
..##...
..###..
.......
.......
......."#.to_string()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Point {
    fn translate(&self, x: i64, y: i64, z: i64, w: i64) -> Self {
        Point{
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
            w: self.w + w,
        }
    }
}

#[derive(Debug)]
struct World {
    points: HashSet<Point>,
    lb: Point,
    ub: Point,
    cycle: usize,
    three_d: bool,
}

impl World {
    fn from_string(data: String, three_d: bool) -> Self {
        let mut points: HashSet<Point> = HashSet::new();
        let mut y = 0;
        let mut y_min = std::i64::MAX;
        let mut y_max = std::i64::MIN;
        let mut x_min = std::i64::MAX;
        let mut x_max = std::i64::MIN;
        for line in data.lines() {
            let mut x = 0;
            for ch in line.trim().chars() {
                match ch {
                    '.' => {},
                    '#' => {
                        if x < x_min {x_min = x};
                        if x > x_max {x_max = x};
                        if y < y_min {y_min = y};
                        if y > y_max {y_max = y};
                        points.insert(Point{x, y, z: 0, w: 0});
                    },
                    _ => panic!("Unknown char {} in line {}", ch, line),
                }
                x += 1;
            }
            y += 1;
        }
        World{
            points,
            lb: Point{x:x_min, y:y_min, z:0, w: 0},
            ub: Point{x:x_max, y:y_max, z:0, w: 0},
            cycle: 0,
            three_d,
        }
    }

    fn to_string(&self) -> String {
        let mut lines: Vec<String> = vec![];
        if self.cycle == 0 {
            lines.push("Before any cycles:\n".to_string())
        } else {
            lines.push(format!("After {} cycle:\n", self.cycle));
        }
        for w in self.lb.w..self.ub.w + 1 {
            for z in self.lb.z..self.ub.z + 1 {
                if self.three_d {
                    lines.push(format!("z={}", z));
                } else {
                    lines.push(format!("z={}, w={}", z, w));
                }
                for y in self.lb.y..self.ub.y + 1 {
                    let mut line: Vec<char> = vec![];
                    for x in self.lb.x..self.ub.x + 1 {
                        if self.points.contains(&Point{x, y, z, w}) {
                            line.push('#');
                        } else {
                            line.push('.');
                        }
                    }
                    lines.push(line.iter().collect());
                }
                lines.push("".to_string());
            }
        }
        lines.push("".to_string());
        lines.join("\n")
    }

    fn count_neighbours(&self, pt: &Point) -> usize {
        let mut neighbours = 0;
        for w in -1..2 {
            if self.three_d && w != 0 { continue; } 
            for z in -1..2 {
                for y in -1..2 {
                    for x in -1..2 {
                        if z == 0 && y == 0 && x == 0 && w == 0 { continue; }
                        if self.points.contains(&pt.translate(x, y, z, w)) {
                            neighbours += 1;
                        }
                    }
                }
            }
        }
        neighbours
    }

    fn evolve(&mut self) {
        let mut points: HashSet<Point> = HashSet::new();
        for w in self.lb.w - 1..self.ub.w + 2 {
            if self.three_d && w != self.lb.w { continue; }
            for z in self.lb.z - 1..self.ub.z + 2 {
                for y in self.lb.y - 1..self.ub.y + 2 {
                    for x in self.lb.x - 1..self.ub.x + 2 {
                        let pt = Point{x, y, z, w};
                        match self.count_neighbours(&pt) {
                            3 => { points.insert(pt); },
                            2 => {
                                if self.points.contains(&pt) {
                                    points.insert(pt);
                                }
                            }
                            _ => {},
                        }
                    }
                }
            }
        }
        self.lb = Point{
            x: std::i64::MAX,
            y: std::i64::MAX,
            z: std::i64::MAX,
            w: std::i64::MAX,
        };
        self.ub = Point{
            x: std::i64::MIN,
            y: std::i64::MIN,
            z: std::i64::MIN,
            w: std::i64::MIN,
        };
        for pt in points.iter() {
            if pt.x < self.lb.x {
                self.lb.x = pt.x;
            } else if pt.x > self.ub.x {
                self.ub.x = pt.x;
            }
            if pt.y < self.lb.y {
                self.lb.y = pt.y;
            } else if pt.y > self.ub.y {
                self.ub.y = pt.y;
            }
            if pt.z < self.lb.z {
                self.lb.z = pt.z;
            } else if pt.z > self.ub.z {
                self.ub.z = pt.z;
            }
            if pt.w < self.lb.w {
                self.lb.w = pt.w;
            } else if pt.w > self.ub.w {
                self.ub.w = pt.w;
            }
        }
        self.cycle += 1;
        if points.is_empty() {
            panic!("Cycle {} caused empty world evolving from:\n{}", self.cycle, self.to_string());
        }
        self.points = points;
    }

    fn size(&self) -> Point {
        Point{
            x: self.ub.x - self.lb.x + 1,
            y: self.ub.y - self.lb.y + 1,
            z: self.ub.z - self.lb.z + 1,
            w: self.ub.w - self.lb.w + 1,
        }
    }
}

fn compare_line(l1: &str, l2: &str) {
    if l1 == l2 {
        println!("{}", l1);
        return;
    }
    let mut cs1 = l1.chars();
    let mut cs2 = l2.chars();
    loop {
        match cs1.next() {
            Some(c1) => {
                match cs2.next() {
                    Some(c2) => {
                        if c1 == c2 {
                            print!("{}", c1);
                        } else {
                            match c1 {
                                'X' => print!("X"),
                                '.' => print!(":"),
                                _ => print!("{}", c1),
                            }
                        }
                    },
                    None => {
                        match c1 {
                            'X' => print!("x"),
                            '.' => print!(";"),
                            _ => print!("!"),
                        }
                    },
                }
            },
            None => {
                match cs2.next() {
                    Some(c2) => {
                        match c2 {
                            'X' => print!("+"),
                            '.' => print!("-"),
                            _ => print!("{}", c2),
                        }
                    },
                    None => {
                        println!("");
                        return;
                    },    
                }
            }
        }
    }
}

fn compare(world1: String, world2: String) {
    let mut lines1 = world1.lines();
    let mut lines2 = world2.lines();
    loop {
        match lines1.next() {
            Some(l1) => {
                match lines2.next() {
                    Some(l2) => compare_line(l1, l2),
                    None => println!("+{}", l1),
                }
            },
            None => {
                match lines2.next() {
                    Some(l2) => println!("-{}", l2),
                    None => return,
                }
            }
        }
    }
}

fn main() {
    let is_3d = false;
    let verbose = false;
    let is_demo = false; 
    let iterations = 6;
    let data = match is_demo { true => load_demo(), false => load_data()};
    let expect = match is_3d { true => load_expected(), false => load_expected_4d()};
    let mut world = World::from_string(data, is_3d);
    let mut history: Vec<String> = vec![];
    history.push(world.to_string());
    println!("{}", world.to_string());
    for _ in 0..iterations {
        world.evolve();
        if verbose {
            history.push(world.to_string());
        }
        println!("Cycle {}", world.cycle);
    }
    if verbose {
        compare(history.join("\n"), expect);
    }
    println!("Active Cubes: {}", world.points.len());
    println!("World dimensions: {:?}", world.size());
}
