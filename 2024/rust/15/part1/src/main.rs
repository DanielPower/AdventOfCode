use std::ops::Add;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
    Box,
}

impl Direction {
    fn delta(&self) -> Point {
        match self {
            Direction::Up => Point(0, -1),
            Direction::Down => Point(0, 1),
            Direction::Left => Point(-1, 0),
            Direction::Right => Point(1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

fn parse_input() -> (Vec<Vec<Tile>>, Vec<Direction>, Point) {
    let mut warehouse: Vec<Vec<Tile>> = Vec::new();
    let mut robot_position = Point(0, 0);
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        warehouse.push(Vec::new());
        for char in line.chars() {
            let row = warehouse.last_mut().unwrap();
            match char {
                '#' => row.push(Tile::Wall),
                '.' => row.push(Tile::Empty),
                'O' => row.push(Tile::Box),
                '@' => {
                    row.push(Tile::Empty);
                    robot_position = Point(row.len() as i32 - 1, warehouse.len() as i32 - 1);
                }
                _ => panic!("Invalid character in input"),
            }
        }
    }

    let mut instructions: Vec<Direction> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        for char in line.chars() {
            let direction = match char {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("Invalid character in input"),
            };
            instructions.push(direction);
        }
    }
    (warehouse, instructions, robot_position)
}

fn tick(warehouse: &mut Vec<Vec<Tile>>, direction: Direction, robot_position: &mut Point) {
    let destination = *robot_position + direction.delta();
    match warehouse[destination.1 as usize][destination.0 as usize] {
        Tile::Wall => (),
        Tile::Box => {
            let mut box_destination = destination + direction.delta();
            loop {
                match warehouse[box_destination.1 as usize][box_destination.0 as usize] {
                    Tile::Wall => break,
                    Tile::Box => box_destination = box_destination + direction.delta(),
                    Tile::Empty => {
                        warehouse[destination.1 as usize][destination.0 as usize] = Tile::Empty;
                        warehouse[box_destination.1 as usize][box_destination.0 as usize] =
                            Tile::Box;
                        *robot_position = destination;
                        break;
                    }
                }
            }
        }
        Tile::Empty => {
            *robot_position = destination;
        }
    }
}

fn gps_value(warehouse: &Vec<Vec<Tile>>) -> usize {
    let mut value = 0;
    for x in 0..warehouse[0].len() {
        for y in 0..warehouse.len() {
            match warehouse[y][x] {
                Tile::Box => value += 100 * y + x,
                _ => (),
            }
        }
    }
    value
}

fn main() {
    let (mut warehouse, instructions, mut robot_position) = parse_input();
    for instruction in instructions {
        tick(&mut warehouse, instruction, &mut robot_position);
    }
    let value = gps_value(&warehouse);
    println!("{}", value);
}
