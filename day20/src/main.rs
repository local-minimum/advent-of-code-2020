use std::collections::HashMap;

fn load_demo() -> String {
    r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#.to_string()
}

fn load_data() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

#[derive(Debug, Clone, PartialEq)]
enum Dir {West, WestFlipped, North, NorthFlipped, East, EastFlipped, South, SouthFlipped}

impl Dir {
    fn is_flipped(&self) -> bool {
        match self {
            Dir::West | Dir::North | Dir::South | Dir::East => false,
            _ => true,
        }
    }

    fn to_ordinal(&self) -> usize {
        match self {
            Dir::North | Dir::NorthFlipped => 0,
            Dir::East | Dir::EastFlipped  => 1,
            Dir::South | Dir::SouthFlipped => 2,
            Dir::West | Dir::WestFlipped => 3,
        }
    }

    fn opposing_ordinal(val: &usize) -> usize {
        match val {
            0 => 2,
            1 => 3,
            2 => 0,
            3 => 1,
            _ => panic!("Unknown ordinal {}", val),
        }
    }
    
    fn ortho_flip(&self, other: &Dir) -> bool {
        match self {
            Dir::East | Dir::EastFlipped => *other == Dir::East || *other == Dir::EastFlipped,
            Dir::West | Dir::WestFlipped => *other == Dir::West || *other == Dir::WestFlipped,
            Dir::South | Dir::SouthFlipped => *other == Dir::South || *other == Dir::SouthFlipped,
            Dir::North | Dir::NorthFlipped => *other == Dir::North|| *other == Dir::NorthFlipped,
        }
    }
}

#[derive(Debug, Clone)]
struct Edge {    
    target: usize,
    from: Dir,
    to: Dir,
}

impl Edge {
    fn is_ortho_flipping(&self) -> bool {
        self.from.ortho_flip(&self.to)
    }

    fn secondary_direction_numeral(&self) -> usize {
        // println!("{:?} {} {}", self, self.from.to_ordinal(), self.to.to_ordinal());
        match (self.from.to_ordinal(), self.to.to_ordinal()) {
            (3, 1) => 0,
            (1, 3) => 2,
            (0, 2) => 1,
            (2, 0) => 3,
            _ => panic!("No secondary dir for {:?}", self),
        }
    }
}

#[derive(Debug)]
struct Tile {
    id: usize,
    tile: Vec<String>,
    sides: HashMap<String, Vec<Dir>>,    
    edges: Vec<Vec<Edge>>,
}

fn sides_from_tile(tile: &Vec<String>) -> HashMap<String, Vec<Dir>> {
    let mut sides: HashMap<String, Vec<Dir>> = HashMap::new();
    // North
    let north: String = tile[0].clone();
    let inv_north = north.chars().rev().collect::<String>();
    sides.insert(north, vec![Dir::North]);
    if sides.contains_key(&inv_north) {
        sides.get_mut(&inv_north).unwrap().push(Dir::NorthFlipped);
    } else {
        sides.insert(inv_north, vec![Dir::NorthFlipped]);
    }
    // South
    let south: String = tile[tile.len() - 1].clone();
    let inv_south= south.chars().rev().collect::<String>();
    if sides.contains_key(&south) {
        sides.get_mut(&south).unwrap().push(Dir::South);
    } else {
        sides.insert(south, vec![Dir::South]);
    }
    if sides.contains_key(&inv_south) {
        sides.get_mut(&inv_south).unwrap().push(Dir::SouthFlipped);
    } else {
        sides.insert(inv_south, vec![Dir::SouthFlipped]);
    }
    // West - East collecting
    let mut west_chars: Vec<char> = vec![];
    let mut east_chars: Vec<char> = vec![];
    for line in tile {
        let chars: Vec<char> = line.chars().collect();
        west_chars.push(chars.first().unwrap().clone());
        east_chars.push(chars.last().unwrap().clone());
    }
    // West
    let west: String = west_chars.iter().collect();
    let inv_west: String = west_chars.into_iter().rev().collect();
    if sides.contains_key(&west) {
        sides.get_mut(&west).unwrap().push(Dir::West);
    } else {
        sides.insert(west, vec![Dir::West]);
    }
    if sides.contains_key(&inv_west) {
        sides.get_mut(&inv_west).unwrap().push(Dir::WestFlipped);
    } else {
        sides.insert(inv_west, vec![Dir::WestFlipped]);
    }
    // East
    let east: String = east_chars.iter().collect();
    let inv_east: String = east_chars.into_iter().rev().collect();
    if sides.contains_key(&east) {
        sides.get_mut(&east).unwrap().push(Dir::East);
    } else {
        sides.insert(east, vec![Dir::East]);
    }
    if sides.contains_key(&inv_east) {
        sides.get_mut(&inv_east).unwrap().push(Dir::EastFlipped);
    } else {
        sides.insert(inv_east, vec![Dir::EastFlipped]);
    }
    sides
}

impl Tile {
    fn from_lines<'a>(
        mut lines: impl std::iter::Iterator<Item=&'a str>
    ) -> Vec<Self> {
        let mut tiles: Vec<Self> = vec![];
        let mut id = 0;
        let mut tile: Vec<String> = vec![];
        loop {
            match lines.next() {
                Some(line) => { if line.trim().is_empty() {
                        let sides = sides_from_tile(&tile);
                        let edges: Vec<Vec<Edge>> = vec![vec![], vec![], vec![], vec![]];
                        tiles.push(Tile{id, tile, sides, edges});
                        id = 0;
                        tile = vec![];
                    } else if line.starts_with("Tile") {
                        let header: Vec<&str> = line.split(" ").collect();
                        id = header[1]
                            .trim()
                            .replace(":", "")
                            .parse()
                            .unwrap()
                    } else {
                        tile.push(line.trim().to_string());
                    }
                },
                None => {
                    let sides = sides_from_tile(&tile);
                    let edges: Vec<Vec<Edge>> = vec![vec![], vec![], vec![], vec![]];
                    tiles.push(Tile{id, tile, sides, edges});
                    return tiles;
                }
            }
        }
    }

    fn possible_connections(&self, other: &Tile) -> Vec<(Dir, Dir)> {
        let mut pairs:  Vec<(Dir, Dir)> = vec![];
        for (k, edges) in self.sides.iter() {
            if other.sides.contains_key(k) {
                for self_e in edges {
                    for other_e in other.sides.get(k).unwrap() {
                        pairs.push(((*self_e).clone(), (*other_e).clone()));
                    }
                }
            }
        }
        pairs
    }

    fn register_connections(&mut self, other_id: usize, connections: &Vec<(Dir, Dir)>) {
        for (from, to) in connections.iter() {
            match from {
                Dir::North | Dir::NorthFlipped => {
                    self.edges[0].push(Edge{target: other_id, from: from.clone(), to: to.clone()});
                }
                Dir::East | Dir::EastFlipped => {
                    self.edges[1].push(Edge{target: other_id, from: from.clone(), to: to.clone()});
                },
                Dir::South | Dir::SouthFlipped => {
                    self.edges[2].push(Edge{target: other_id, from: from.clone(), to: to.clone()});
                },
                Dir::West | Dir::WestFlipped => {
                    self.edges[3].push(Edge{target: other_id, from: from.clone(), to: to.clone()});
                }
            }
        }
    }

    fn register_connections_rev(&mut self, other_id: usize, connections: &Vec<(Dir, Dir)>) {
        for (from, to) in connections.iter() {
            match to {
                Dir::North | Dir::NorthFlipped => {
                    self.edges[0].push(Edge{target: other_id, from: to.clone(), to: from.clone()});
                }
                Dir::East | Dir::EastFlipped => {
                    self.edges[1].push(Edge{target: other_id, from: to.clone(), to: from.clone()});
                },
                Dir::South | Dir::SouthFlipped => {
                    self.edges[2].push(Edge{target: other_id, from: to.clone(), to: from.clone()});
                },
                Dir::West | Dir::WestFlipped => {
                    self.edges[3].push(Edge{target: other_id, from: to.clone(), to: from.clone()});
                }
            }
        }
    }


    fn unresolved_connections(&self) -> usize {
        self
            .edges
            .iter()
            .filter(|e| e.len() > 1)
            .count()
    }

    fn connections(&self) -> usize {
        self
            .edges
            .iter()
            .filter(|e| e.len() > 0)
            .count()
    }

    fn north_west_corner(&self) -> bool {
        self.edges[0].len() == 0 && self.edges[1].len() > 0 && self.edges[2].len() > 0 && self.edges[3].len() == 0
    }

    fn prune_other_edges(&mut self, dir: Dir, target: usize) {
        if self.id == 1427 {println!("Removing not {:?} to {}", dir, target)}
        let i = dir.to_ordinal();
        for j in (0..self.edges[i].len()).rev() {
            let edge = self.edges.get(i).unwrap().get(j).unwrap();
            if edge.target != target || dir != edge.from {
                self.edges.get_mut(i).unwrap().remove(j);
            }
        }
    }

    fn prune_edges(&mut self, first: &bool) {
        if *first {
            for i in 0..4 {
                for j in (0..self.edges[i].len()).rev() {
                    let edge = self.edges.get(i).unwrap().get(j).unwrap();
                    if edge.from.is_flipped() {
                        self.edges.get_mut(i).unwrap().remove(j);
                    }
                }
            }
        } else {
            let mut removals: Vec<(usize, usize)> = vec![];
            for i in 0..4 {
                let edges = self.edges.get(i).unwrap();
                if edges.len() != 1 { continue; }
                let edge = edges.first().unwrap();
                let opposing = Dir::opposing_ordinal(&edge.from.to_ordinal());
                for j in (0..self.edges.get(opposing).unwrap().len()).rev() {
                    let oppo = self.edges.get(opposing).unwrap().get(j).unwrap();
                    if oppo.from.is_flipped() != edge.from.is_flipped() {
                        removals.push((opposing, j));
                    }
                }
                let ortho_twist = edge.is_ortho_flipping();
                for j in 0..4 {
                    if j == i || j == opposing { continue; }
                    for k in (0..self.edges.get(j).unwrap().len()).rev() {
                        let ortho= self.edges.get(j).unwrap().get(k).unwrap();
                        if (ortho.from.is_flipped() == edge.from.is_flipped()) != ortho_twist {
                            removals.push((j, k));
                        }
                    }
                }
                for (i, j) in removals {
                    self.edges.get_mut(i).unwrap().remove(j);
                }
                break;
            }

        }

    }

    fn resolved_connections(&self) -> Vec<Edge> {
        let mut ret: Vec<Edge> = vec![];
        for edges in self.edges.iter() {
            if edges.len() != 1 { continue; }
            for e in edges {
                ret.push(e.clone());
            }
        }
        ret
    }

    fn out_edge(&self, in_edge: &Edge) -> Option<Edge> {
        let out = Dir::opposing_ordinal(&in_edge.to.to_ordinal());
        if self.edges[out].len() == 1 {
            return Some(self.edges[out].first().unwrap().clone());
        }
        None
    }

    fn rotate_dir_left(&self, dir: &Dir) -> Option<Edge> {
        let mut out = dir.to_ordinal();
        if out == 0 {
            out = 3;
        } else {
            out -= 1;            
        }
        if self.edges[out].len() == 1 {
            return Some(self.edges[out].first().unwrap().clone());
        }
        None
    }

    fn rotate_dir_right(&self, dir: &Dir) -> Option<Edge> {
        let mut out = dir.to_ordinal();
        out += 1;
        if out == 4 {
            out = 0;
        }
        if self.edges[out].len() == 1 {
            return Some(self.edges[out].first().unwrap().clone());
        }
        None
    }

    fn line(&self, in_edge: &Edge, row: &usize, flipped: &bool) -> (String, usize) {
        let idx = match flipped { true => self.tile.len() - (row + 1), false => row.clone()};
        let msg = match in_edge.to {
            Dir::West | Dir::WestFlipped => self.tile[idx].clone(),
            Dir::East | Dir::EastFlipped => self.tile[idx].chars().rev().collect(),
            Dir::South | Dir::SouthFlipped => self.tile
                .iter()
                .map(|l| l.chars().enumerate().filter(|(i, _)| *i == idx).map(|(_, c)| c).collect::<String>())
                .collect(),
            _ => self.tile
                .iter()
                .map(|l| l.chars().enumerate().filter(|(i, _)| *i == idx).map(|(_, c)| c).collect::<String>())
                .rev()
                .collect(),
        };
        return (msg, idx)
    }
}

struct Image {
    tiles: HashMap<usize, Tile>,
}

impl Image {
    fn print_edgemap(&self, edge: Edge) {
        let mut tile = self.tiles.get(&edge.target).unwrap();
        let mut in_edge = tile.out_edge(&edge).unwrap();
        let mut sec_in_edge = tile.rotate_dir_right(&in_edge.from);
        loop {
            print!("{}\t{:?}->{:?}\t", tile.id, in_edge.from, in_edge.to);
            tile = self.tiles.get(&in_edge.target).unwrap();
            match tile.out_edge(&in_edge) {
                Some(e) => {
                    in_edge = e;
                }
                None => {
                    println!("\t{}",tile.id);
                    match sec_in_edge {
                        Some(e2) => {
                            println!("{:?} v {:?}", e2.from, e2.to);
                            tile = self.tiles.get(&e2.target).unwrap();
                            sec_in_edge = tile.out_edge(&e2);
                            in_edge = tile.rotate_dir_right(&e2.to).unwrap()
                        },
                        None => {
                            break;
                        } 
                    }
                }
            }
        }
    }

    fn print_tiled(&self, edge: Edge) {
        let mut tile = self.tiles.get(&edge.target).unwrap();
        let mut in_edge = edge;
        let mut sec_in_edge = tile.rotate_dir_right(&in_edge.from);
        loop {
            for idx in 0..tile.tile.len() {
                let (l, mut row) = tile.line(&in_edge, &idx, &false);
                print!("{} ", l);
                match tile.out_edge(&in_edge) {
                    Some(mut out_edge) => {
                        let mut flipping = out_edge.to.is_flipped();
                        loop {
                            let next = self.tiles.get(&out_edge.target).unwrap();
                            let (line, n_row) = next.line(&out_edge, &row, &flipping);
                            print!("{} ", line);
                            match next.out_edge(&out_edge) {
                                Some(e) => {
                                    flipping = out_edge.to.is_flipped() == out_edge.to.is_flipped();
                                    out_edge = e;
                                    row = n_row;
                                },
                                None => break,
                            }
                        }
                    },
                    None => {},
                }
                println!();
            }
            match sec_in_edge {
                Some(e) => {
                    //println!("{} -> {} {:?}", tile.id, e.target, e);
                    tile = self.tiles.get(&e.target).unwrap();
                    sec_in_edge = tile.out_edge(&e);
                    println!();
                },
                None => break,
            }
        }
    }
}

fn main() {
    let is_demo = true;
    let data = match is_demo {true => load_demo(), false => load_data()};
    let lines = data.lines();
    let mut tiles = Tile::from_lines(lines);
    for i in 0..tiles.len() {
        for j in i+1..tiles.len() {
            let i_id = tiles[i].id.clone();
            let j_id = tiles[j].id.clone();
            let connections = tiles[i].possible_connections(&tiles[j]);
            tiles.get_mut(i).unwrap().register_connections(j_id, &connections);
            tiles.get_mut(j).unwrap().register_connections_rev(i_id, &connections);
        }
    }

    let mut corners: Vec<usize> = vec![];
    for tile in tiles.iter() {
        println!("Tile {} has {} possible connections, {} amibigous", tile.id, tile.connections(), tile.unresolved_connections());
        if tile.connections() == 2 {
            corners.push(tile.id.clone());
        }
    }
    println!("Part 1: {} / {:?}", corners.iter().fold(1, |a, b| a * b), corners);
    let seed_id: usize = tiles.iter().filter(|t| t.north_west_corner()).next().unwrap().id.clone();
    let mut tile_map: HashMap<usize, Tile> = HashMap::new();
    for tile in tiles {
        tile_map.insert(tile.id.clone(), tile);
    }
    let mut seeds: Vec<usize> = vec![seed_id];
    let mut first: bool = true;
    println!("Using {} as Upper Left", seed_id);
    loop {
        match seeds.pop() {
            Some(seed_id) => {
                tile_map.get_mut(&seed_id).unwrap().prune_edges(&first);
                first = false;
                for e in tile_map.get(&seed_id).unwrap().resolved_connections() {
                    if e.target == 1427 { println!("{}: {:?} / {:?}", seed_id, e, tile_map.get(&seed_id).unwrap().edges);}
                    tile_map.get_mut(&e.target).unwrap().prune_other_edges(e.to, seed_id.clone());
                    if tile_map.get(&e.target).unwrap().unresolved_connections() > 0 {
                        seeds.push(e.target);
                    }
                }
                // println!("{:?}", seeds);
                // println!("{}: {:?}", seed_id, tileMap.get(&seed_id).unwrap().edges);
            },
            None => {
                break;
            }
        }
    }
    for (k, v) in tile_map.iter() {
        println!("{}: {} / {}", k, v.resolved_connections().len(), v.unresolved_connections())
    }
    let img = Image{tiles: tile_map};
    img.print_tiled(Edge{target: seed_id, from: Dir::East, to: Dir::West});
    img.print_edgemap(Edge{target: seed_id, from: Dir::East, to: Dir::West });
}
