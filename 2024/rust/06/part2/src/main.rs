use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input<R: std::io::Read>(reader: R) -> ((i32, i32), Vec<(i32, i32)>) {
    let mut y = 0;
    let mut x = 0;
    let mut last_char = 'x';
    let mut guard: (i32, i32) = (0, 0);
    let mut obstacles: Vec<(i32, i32)> = Vec::new();
    for byte in reader.bytes() {
        match byte {
            Ok(b) => {
                let char = b as char;
                match char {
                    '\n' => {
                        if last_char == '\n' {
                            break;
                        }
                        y += 1;
                        x = -1;
                    }
                    '^' => {
                        guard = (x as i32, y);
                    }
                    '#' => obstacles.push((x as i32, y)),
                    _ => (),
                }
                last_char = char;
            }
            Err(e) => panic!("Error: {}", e),
        }
        x += 1;
    }
    println!("Guard: {:?}, Obstacles: {:?}", guard, obstacles);
    (guard, obstacles)
}

fn next_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn next_obstacle(
    obstacles: Vec<(i32, i32)>,
    (guard_x, guard_y): (i32, i32),
    direction: Direction,
) -> Option<(i32, i32)> {
    match direction {
        Direction::Up => {
            let mut obstacles_ahead = obstacles
                .into_iter()
                .filter(|(x, y)| *x == guard_x && *y < guard_y)
                .collect::<Vec<(i32, i32)>>();
            obstacles_ahead.sort_by(|(_, y1), (_, y2)| y1.cmp(y2));
            match obstacles_ahead.first() {
                Some((_, y)) => Some((guard_x, y + 1)),
                None => None,
            }
        }
        Direction::Down => {
            let mut obstacles_ahead = obstacles
                .into_iter()
                .filter(|(x, y)| *x == guard_x && *y > guard_y)
                .collect::<Vec<(i32, i32)>>();
            obstacles_ahead.sort_by(|(_, y1), (_, y2)| y1.cmp(y2));
            match obstacles_ahead.first() {
                Some((_, y)) => Some((guard_x, y - 1)),
                None => None,
            }
        }
        Direction::Left => {
            let mut obstacles_ahead = obstacles
                .into_iter()
                .filter(|(x, y)| *y == guard_y && *x < guard_x)
                .collect::<Vec<(i32, i32)>>();
            obstacles_ahead.sort_by(|(x1, _), (x2, _)| x1.cmp(x2));
            match obstacles_ahead.first() {
                Some((x, _)) => Some((x + 1, guard_y)),
                None => None,
            }
        }
        Direction::Right => {
            let mut obstacles_ahead = obstacles
                .into_iter()
                .filter(|(x, y)| *y == guard_y && *x > guard_x)
                .collect::<Vec<(i32, i32)>>();
            obstacles_ahead.sort_by(|(x1, _), (x2, _)| x1.cmp(x2));
            match obstacles_ahead.first() {
                Some((x, _)) => Some((x - 1, guard_y)),
                None => None,
            }
        }
    }
}

fn next_potential_obstacle(
    obstacles: Vec<(i32, i32)>,
    (guard_x, guard_y): (i32, i32),
    direction: Direction,
) -> Option<(i32, i32)> {
    println!("{:?}, {:?}, {:?}", obstacles, (guard_x, guard_y), direction);
    let sorted_obstacles = match direction {
        Direction::Up => match obstacles.clone()
            .into_iter()
            .filter(|(x, y)| *x > guard_x && *y < guard_y)
            .collect::<Vec<(i32, i32)>>().sort_by(|(_, y1), (_, y2)| y1.cmp(y2))
        Direction::Down => {
            match obstacles
                .into_iter()
                .find(|(x, y)| *x < guard_x && *y == guard_y)
            {
                Some((_, y)) => Some((guard_x, y + 1)),
                None => None,
            }
        }
        Direction::Left => {
            match obstacles
                .into_iter()
                .find(|(x, y)| *y < guard_y && *x == guard_x)
            {
                Some((x, _)) => Some((x - 1, guard_y)),
                None => None,
            }
        }
        Direction::Right => {
            match obstacles
                .into_iter()
                .find(|(x, y)| *y > guard_y && *x == guard_x)
            {
                Some((x, _)) => Some((x + 1, guard_y)),
                None => None,
            }
        }
    }
}

fn find_loop(
    obstacles: Vec<(i32, i32)>,
    position_initial: (i32, i32),
    direction_initial: Direction,
) -> bool {
    let mut direction = direction_initial;
    let mut position = position_initial;
    let mut visited: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();
    loop {
        let next_position = match next_obstacle(obstacles.clone(), position, direction) {
            Some(p) => p,
            None => return false,
        };
        direction = next_direction(direction);
        if visited.contains_key(&position) && visited[&position].contains(&direction) {
            return true;
        }
        visited
            .entry(position)
            .or_insert_with(HashSet::new)
            .insert(direction);
        position = next_position;
    }
}

fn main() {
    let (guard, mut obstacles) = parse_input(std::io::stdin());
    let mut direction = Direction::Up;
    let mut position = guard;
    let mut valid_obstacles: HashSet<(i32, i32)> = HashSet::new();
    loop {
        println!("Start: {:?}", position);
        let next_position = match next_potential_obstacle(obstacles.clone(), position, direction) {
            None => break,
            Some(p) => p,
        };
        println!("Next Potential Obstacle: {:?}", next_position);
        direction = next_direction(direction);
        if next_position != guard {
            obstacles.push(next_position);
            let has_loop = find_loop(obstacles.clone(), guard, Direction::Up);
            if has_loop {
                valid_obstacles.insert(next_position);
            }
        }
        position = next_position;
    }
    println!("{}", valid_obstacles.len());
}
