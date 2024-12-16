use std::ops::Add;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
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
                '#' => {
                    row.push(Tile::Wall);
                    row.push(Tile::Wall);
                }
                '.' => {
                    row.push(Tile::Empty);
                    row.push(Tile::Empty);
                }
                'O' => {
                    row.push(Tile::BoxLeft);
                    row.push(Tile::BoxRight);
                }
                '@' => {
                    row.push(Tile::Empty);
                    row.push(Tile::Empty);
                    robot_position = Point(row.len() as i32 - 2, warehouse.len() as i32 - 1);
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

fn can_push(warehouse: &Vec<Vec<Tile>>, direction: Direction, target: Point) -> bool {
    match warehouse[target.1 as usize][target.0 as usize] {
        Tile::Wall => false,
        box_side @ (Tile::BoxLeft | Tile::BoxRight) => match direction {
            Direction::Left | Direction::Right => {
                can_push(warehouse, direction, target + direction.delta())
            }
            Direction::Up | Direction::Down => {
                let next_targets = match box_side {
                    Tile::BoxLeft => (
                        target + direction.delta(),
                        target + direction.delta() + Direction::Right.delta(),
                    ),
                    Tile::BoxRight => (
                        target + direction.delta(),
                        target + direction.delta() + Direction::Left.delta(),
                    ),
                    _ => unreachable!(),
                };
                can_push(warehouse, direction, next_targets.0)
                    && can_push(warehouse, direction, next_targets.1)
            }
        },
        Tile::Empty => true,
    }
}

fn push(warehouse: &mut Vec<Vec<Tile>>, direction: Direction, target: Point, value: Tile) {
    match warehouse[target.1 as usize][target.0 as usize] {
        box_side @ (Tile::BoxLeft | Tile::BoxRight) => match direction {
            Direction::Left | Direction::Right => {
                push(warehouse, direction, target + direction.delta(), box_side);
                warehouse[target.1 as usize][target.0 as usize] = value;
            }
            Direction::Up | Direction::Down => {
                let targets = match box_side {
                    Tile::BoxLeft => (target, target + Direction::Right.delta()),
                    Tile::BoxRight => (target + Direction::Left.delta(), target),
                    _ => unreachable!(),
                };
                push(
                    warehouse,
                    direction,
                    targets.0 + direction.delta(),
                    Tile::BoxLeft,
                );
                push(
                    warehouse,
                    direction,
                    targets.1 + direction.delta(),
                    Tile::BoxRight,
                );
                warehouse[targets.0 .1 as usize][targets.0 .0 as usize] = Tile::Empty;
                warehouse[targets.1 .1 as usize][targets.1 .0 as usize] = Tile::Empty;
                warehouse[target.1 as usize][target.0 as usize] = value;
            }
        },
        Tile::Empty => {
            warehouse[target.1 as usize][target.0 as usize] = value;
        }
        Tile::Wall => (),
    }
}

fn tick(warehouse: &mut Vec<Vec<Tile>>, direction: Direction, robot_position: &mut Point) {
    let destination = *robot_position + direction.delta();
    match warehouse[destination.1 as usize][destination.0 as usize] {
        Tile::Wall => (),
        Tile::BoxLeft | Tile::BoxRight => {
            if can_push(warehouse, direction, destination) {
                push(warehouse, direction, destination, Tile::Empty);
                *robot_position = destination;
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
                Tile::BoxLeft => value += 100 * y + x,
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
    println!("{}", gps_value(&warehouse));
}
