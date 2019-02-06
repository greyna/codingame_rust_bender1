use std::io;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, PartialEq)]
enum Cell {
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
    fn new(c: &char) -> Cell {
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
            _ => { eprintln!("char '{}' unsupported", c); panic!() },
        }
    }
}

#[derive(Debug)]
struct Map(Vec<Vec<Cell>>);

struct Position(u32, u32);

impl Map {
    fn new(rows: Vec<String>) -> Map {
        let mut cells = Vec::new();
        for (i, row) in rows.iter().enumerate() {
            cells.push(Vec::new());
            for cell_char in row.chars() {
                cells[i].push(Cell::new(&cell_char));
            }
        }
        Map(cells)
    }

    fn get(&self, pos: Position) -> &Cell {
        &self.0[pos.0 as usize][pos.1 as usize]
    }

    fn surroundings(&self, pos: Position) -> HashMap<Direction, &Cell> {
        let mut result = HashMap::new();
        result
    }

    fn find(&self, cell_to_find: Cell) -> Vec<Position> {
        self.0.iter().enumerate()
            .flat_map(|(x, row)| row.iter().enumerate()
                .filter(|(_, cell)| cell_to_find == **cell)
                .map(|(y, _)| Position(x.clone() as u32, y.clone() as u32)))
            .collect()
    }
}

struct Bender {
    pos: Position,
    dir: Direction,
    drunk: bool,
    inverted: bool,
}

impl Bender {
    fn new(map: &Map) -> Bender {
        Bender {
            pos: map.find(Cell::Start).first().unwrap(),
            dir: Direction::South,
            drunk: false,
            inverted: false,
        }
    }
}

fn parse_input() -> (u32, u32, Vec<String>) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let rows_count: u32 = inputs[0].trim().parse().unwrap();
    let columns_count: u32 = inputs[1].trim().parse().unwrap();
    
    let mut rows = Vec::new();
    for _ in 0..rows_count as u32 {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_right().to_string();
        rows.push(row);
    }

    (rows_count, columns_count, rows)
}

fn solve(map: Map) -> String {
    String::from("LOOP")
}

fn main() {
    let (_, _, rows) = parse_input();
    let map = Map::new(rows);
    eprintln!("{:?}", map);
    
    println!("{}", solve(map));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_display() {
        let input =
"#####
#@  #
#   #
#  $#
#####";
        let rows: Vec<String> = input.split("\n").map(|s| String::from(s)).collect();
        let map = Map::new(rows);
        eprintln!("{:?}", map);
    }

    #[test]
    fn test_01() {
        let input =
"#####
#@  #
#   #
#  $#
#####";
        let rows: Vec<String> = input.split("\n").map(|s| String::from(s)).collect();

        let expectation =
"SOUTH
SOUTH
EAST
EAST";

        let output = solve(Map::new(rows));
        assert_eq!(output, expectation);
    }
}