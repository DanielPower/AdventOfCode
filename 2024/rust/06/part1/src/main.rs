enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Grid {
    items: Vec<char>,
    width: usize,
    height: usize,
    guard: usize,
}

impl Grid {
    fn from_reader<R: std::io::Read>(reader: R) -> Self {
        let mut items: Vec<char> = Vec::new();
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
                            items.push('x');
                        }
                        _ => {
                            items.push(char);
                        }
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
    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x < self.width && y < self.height {
            Some(self.items[y * self.width + x])
        } else {
            None
        }
    }
    fn set(&mut self, x: usize, y: usize, c: char) {
        self.items[y * self.width + x] = c;
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

fn next_position(x: i32, y: i32, direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn main() {
    let mut grid = Grid::from_reader(std::io::stdin());
    let mut direction = Direction::Up;
    let mut total = 1; // Guard's starting position
    let mut x: i32 = (grid.guard % grid.width).try_into().unwrap();
    let mut y: i32 = (grid.guard / grid.width).try_into().unwrap();
    loop {
        let (next_x, next_y) = next_position(x, y, &direction);
        let current_char = grid.get(x as usize, y as usize).unwrap();
        if current_char == '.' {
            total += 1;
            grid.set(x as usize, y as usize, 'x');
        }
        match grid.get(next_x as usize, next_y as usize) {
            Some('#') => direction = next_direction(direction),
            Some(_) => {
                x = next_x;
                y = next_y;
            }
            None => {
                break;
            }
        }
    }
    println!("{}", total);
}
