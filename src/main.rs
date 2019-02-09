#![warn(clippy::style)]
#![warn(clippy::correctness)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]

use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::io;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::South => "SOUTH",
                Direction::East => "EAST",
                Direction::North => "NORTH",
                Direction::West => "WEST",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Invalid,
    Start,
    Wall,
    Empty,
    Suicide,
    Obstacle,
    Dir(Direction),
    Invertor,
    Beer,
    Teleporter,
}

impl Cell {
    fn new(c: char) -> Cell {
        match c {
            '@' => Cell::Start,
            '#' => Cell::Wall,
            ' ' => Cell::Empty,
            '$' => Cell::Suicide,
            'X' => Cell::Obstacle,
            'S' => Cell::Dir(Direction::South),
            'N' => Cell::Dir(Direction::North),
            'E' => Cell::Dir(Direction::East),
            'W' => Cell::Dir(Direction::West),
            'I' => Cell::Invertor,
            'B' => Cell::Beer,
            'T' => Cell::Teleporter,
            _ => panic!(format!("char '{}' unsupported", c)),
        }
    }
}

#[derive(Debug, Clone)]
struct Map(Vec<Vec<Cell>>);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position(usize, usize);

impl Map {
    fn new(rows_count: usize, columns_count: usize, rows: Vec<String>) -> Map {
        let mut cells = vec![vec![Cell::Invalid; rows_count]; columns_count];
        for (y, row) in rows.iter().enumerate() {
            for (x, cell_char) in row.chars().enumerate() {
                cells[x][y] = Cell::new(cell_char);
            }
        }
        Map(cells)
    }

    fn get(&self, pos: Position) -> &Cell {
        &self.0[pos.0][pos.1]
    }

    fn set(&mut self, pos: Position, cell: Cell) {
        self.0[pos.0][pos.1] = cell;
    }

    fn surroundings(&self, pos: Position) -> HashMap<Direction, &Cell> {
        let mut result = HashMap::new();
        result.insert(Direction::North, &self.0[pos.0][pos.1 - 1]);
        result.insert(Direction::South, &self.0[pos.0][pos.1 + 1]);
        result.insert(Direction::East, &self.0[pos.0 + 1][pos.1]);
        result.insert(Direction::West, &self.0[pos.0 - 1][pos.1]);
        result
    }

    fn find(&self, cell_to_find: Cell) -> Vec<Position> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, cell)| cell_to_find == **cell)
                    .map(move |(y, _)| Position(x, y))
            })
            .collect()
    }
}

#[derive(Debug)]
struct Bender {
    pos: Position,
    dir: Direction,
    drunk: bool,
    inverted: bool,
    dead: bool,
}

impl Bender {
    fn new(map: &Map) -> Bender {
        Bender {
            pos: *map.find(Cell::Start).first().unwrap(),
            dir: Direction::South,
            drunk: false,
            inverted: false,
            dead: false,
        }
    }

    fn step(&mut self, map: &mut Map) -> Direction {
        self.turn(map, true);
        let dir = self.dir;
        self.move_forward();
        self.apply_cell_effect(map);
        dir
    }

    fn turn(&mut self, map: &Map, first_turn: bool) {
        let cell = map.surroundings(self.pos)[&self.dir];
        if *cell == Cell::Wall || (*cell == Cell::Obstacle && !self.drunk) {
            self.dir = self.next_dir(first_turn);
            self.turn(map, false);
        }
    }

    fn next_dir(&self, first_turn: bool) -> Direction {
        match (first_turn, self.inverted) {
            (true, false) => Direction::South,
            (false, false) => match self.dir {
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::North => Direction::West,
                Direction::West => panic!("Will infinitely loop between directions"),
            },
            (true, true) => Direction::West,
            (false, true) => match self.dir {
                Direction::West => Direction::North,
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => panic!("Will infinitely loop between directions"),
            },
        }
    }

    fn move_forward(&mut self) {
        match self.dir {
            Direction::South => self.pos.1 += 1,
            Direction::East => self.pos.0 += 1,
            Direction::North => self.pos.1 -= 1,
            Direction::West => self.pos.0 -= 1,
        }
    }

    fn apply_cell_effect(&mut self, map: &mut Map) {
        let cell = map.get(self.pos).clone();
        match cell {
            Cell::Suicide => self.dead = true,
            Cell::Dir(dir) => self.dir = dir,
            Cell::Beer => self.drunk = !self.drunk,
            Cell::Invertor => self.inverted = !self.inverted,
            Cell::Teleporter => {
                for pos in map.find(Cell::Teleporter) {
                    if pos != self.pos {
                        self.pos = pos;
                        break;
                    }
                }
            }
            Cell::Obstacle => map.set(self.pos, Cell::Empty),
            _ => (),
        }
    }
}

fn parse_input() -> (usize, usize, Vec<String>) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let rows_count: usize = inputs[0].trim().parse().unwrap();
    let columns_count: usize = inputs[1].trim().parse().unwrap();

    let mut rows = Vec::new();
    for _ in 0..rows_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_right().to_string();
        rows.push(row);
    }

    (rows_count, columns_count, rows)
}

fn solve(map: Map) -> String {
    let mut directions = Vec::new();
    let mut bender = Bender::new(&map);
    eprintln!("INIT: {:?}", &bender);
    let mut map = map.clone();

    let mut step = 0;
    while !bender.dead {
        let dir = bender.step(&mut map);
        directions.push(dir);

        eprintln!("STEP {}: Moved {:?} with {:?}", step, &dir, &bender);
        step += 1;
        if step > 10000 {
            return String::from("LOOP\n");
        }
    }

    let mut result = String::new();
    for dir in directions.iter() {
        writeln!(&mut result, "{}", dir).unwrap();
    }
    result
}

fn main() {
    let (rows_count, columns_count, rows) = parse_input();
    let map = Map::new(rows_count, columns_count, rows);
    eprintln!("{:?}", map);

    print!("{}", solve(map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_display() {
        let input = "#####
#@  #
#   #
#  $#
#####";
        let rows: Vec<String> = input.split('\n').map(String::from).collect();
        let map = Map::new(5, 5, rows);
        eprintln!("{:?}", map);
    }

    #[test]
    fn test_01() {
        let input = "#####
#@  #
#   #
#  $#
#####";
        let rows: Vec<String> = input.split('\n').map(String::from).collect();

        let expectation = "SOUTH
SOUTH
EAST
EAST\n";

        let output = solve(Map::new(5, 5, rows));
        assert_eq!(output, expectation);
    }

    #[test]
    fn test_02() {
        let input = "########
# @    #
#     X#
# XXX  #
#   XX #
#   XX #
#     $#
########";
        let rows: Vec<String> = input.split('\n').map(String::from).collect();

        let expectation = "SOUTH
EAST
EAST
EAST
SOUTH
EAST
SOUTH
SOUTH
SOUTH\n";

        let output = solve(Map::new(8, 8, rows));
        assert_eq!(output, expectation);
    }

    #[test]
    fn test_05() {
        let input = "##########
#        #
#  S   W #
#        #
#  $     #
#        #
#@       #
#        #
#E     N #
##########";
        let rows: Vec<String> = input.split('\n').map(String::from).collect();

        let expectation = "SOUTH
SOUTH
EAST
EAST
EAST
EAST
EAST
EAST
NORTH
NORTH
NORTH
NORTH
NORTH
NORTH
WEST
WEST
WEST
WEST
SOUTH
SOUTH\n";

        let output = solve(Map::new(10, 10, rows));
        assert_eq!(output, expectation);
    }

    #[test]
    fn test_09() {
        let input = "##########
#        #
#  @     #
#  B     #
#  S   W #
# XXX    #
#  B   N #
# XXXXXXX#
#       $#
##########";
        let rows: Vec<String> = input.split('\n').map(String::from).collect();

        let expectation = "SOUTH
SOUTH
SOUTH
SOUTH
EAST
EAST
EAST
EAST
NORTH
NORTH
WEST
WEST
WEST
WEST
SOUTH
SOUTH
SOUTH
SOUTH
EAST
EAST
EAST
EAST
EAST\n";

        let output = solve(Map::new(10, 10, rows));
        assert_eq!(output, expectation);
    }

    #[test]
    fn test_11() {
        let input = "###############
#      IXXXXX #
#  @          #
#E S          #
#             #
#  I          #
#  B          #
#  B   S     W#
#  B   T      #
#             #
#         T   #
#         B   #
#N          W$#
#        XXXX #
###############";
        let rows: Vec<String> = input.split('\n').map(String::from).collect();

        let expectation = "LOOP\n";

        let output = solve(Map::new(15, 15, rows));
        assert_eq!(output, expectation);
    }
}
