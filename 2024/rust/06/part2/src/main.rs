use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Empty,
    Obstacle,
}

struct Grid {
    items: Vec<Tile>,
    width: usize,
    height: usize,
    guard: usize,
}

impl Grid {
    fn from_reader<R: std::io::Read>(reader: R) -> Self {
        let mut items: Vec<Tile> = Vec::new();
        let mut height = 0;
        let mut last_char = 'x';
        let mut guard = 0;
        for byte in reader.bytes() {
            match byte {
                Ok(b) => {
                    let char = b as char;
                    match char {
                        '\n' => {
                            if last_char == '\n' {
                                break;
                            }
                            height += 1;
                        }
                        '^' => {
                            guard = items.len();
                            items.push(Tile::Empty);
                        }
                        '.' => items.push(Tile::Empty),
                        '#' => items.push(Tile::Obstacle),
                        _ => panic!("Unexpected character: {}", char),
                    }
                    last_char = char;
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        let width = items.len() / height;
        Self {
            items,
            width,
            height,
            guard,
        }
    }
    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            width: self.width,
            height: self.height,
            guard: self.guard,
        }
    }
}

fn next_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn get_next_position(grid: &Grid, position: usize, direction: Direction) -> Option<usize> {
    let x = (position % grid.width).try_into().unwrap();
    let y = (position / grid.width).try_into().unwrap();
    match direction {
        Direction::Up => {
            if y < 1 {
                None
            } else {
                Some(grid.get_index(x, y - 1))
            }
        }
        Direction::Down => {
            if y >= grid.height - 1 {
                None
            } else {
                Some(grid.get_index(x, y + 1))
            }
        }
        Direction::Left => {
            if x < 1 {
                None
            } else {
                Some(grid.get_index(x - 1, y))
            }
        }
        Direction::Right => {
            if x >= grid.width - 1 {
                None
            } else {
                Some(grid.get_index(x + 1, y))
            }
        }
    }
}

fn find_loop(grid: &Grid, position_initial: usize, direction_initial: Direction) -> bool {
    let mut direction = direction_initial;
    let mut position = position_initial;
    let mut visited: Vec<HashSet<Direction>> = Vec::new();
    for _ in 0..grid.items.len() {
        visited.push(HashSet::new());
    }
    loop {
        let next_position = match get_next_position(&grid, position, direction) {
            Some(p) => p,
            None => return false,
        };
        if matches!(grid.items[next_position], Tile::Obstacle) {
            direction = next_direction(direction);
            continue;
        }
        if visited[position].contains(&direction) {
            return true;
        }
        visited[position].insert(direction);
        position = next_position;
    }
}

fn main() {
    let grid = Grid::from_reader(std::io::stdin());
    let mut direction = Direction::Up;
    let mut position = grid.guard;
    let mut valid_obstacles: HashSet<usize> = HashSet::new();
    loop {
        let next_position = match get_next_position(&grid, position, direction) {
            None => break,
            Some(p) => p,
        };
        if matches!(grid.items[next_position], Tile::Obstacle) {
            direction = next_direction(direction);
            continue;
        }
        let mut modified_grid = grid.clone();
        if next_position != grid.guard {
            modified_grid.items[next_position] = Tile::Obstacle;
            let has_loop = find_loop(&modified_grid, grid.guard, Direction::Up);
            if has_loop {
                valid_obstacles.insert(next_position);
            }
        }
        position = next_position;
    }
    println!("{}", valid_obstacles.len());
}
