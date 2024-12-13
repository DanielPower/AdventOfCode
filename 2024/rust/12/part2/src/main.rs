fn parse_input() -> Vec<Vec<char>> {
    let mut plants: Vec<Vec<char>> = Vec::new();
    for (index, line) in std::io::stdin().lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        plants.push(Vec::new());
        for c in line.chars() {
            plants[index].push(c);
        }
    }
    plants
}

fn get_valid_neighbors(x: i32, y: i32, gardens: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut neighbors: Vec<(i32, i32)> = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < (gardens[0].len() - 1) as i32 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < (gardens.len() - 1) as i32 {
        neighbors.push((x, y + 1));
    }
    let value = gardens[y as usize][x as usize];
    neighbors = neighbors
        .into_iter()
        .filter(|(nx, ny)| gardens[*ny as usize][*nx as usize] == value)
        .collect();
    neighbors
}

fn flood_fill(
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    garden: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> (usize, usize) {
    visited[y as usize][x as usize] = true;
    let neighbors = get_valid_neighbors(x, y, garden);
    let mut area = 1;
    let mut perimeter = match neighbors.len() {
        0 => 4,
        1 => 2,
        2 => {
            if neighbors[0].0 == neighbors[1].0 || neighbors[0].1 == neighbors[1].1 {
                0
            } else {
                let dx = neighbors[0].0 as i32 + neighbors[1].0 as i32 - (x as i32) * 2;
                let dy = neighbors[0].1 as i32 + neighbors[1].1 as i32 - (y as i32) * 2;
                if garden[y as usize][x as usize] == garden[(y + dy) as usize][(x + dx) as usize] {
                    1
                } else {
                    2
                }
            }
        }
        3 => {
            let ax = neighbors[0].0 + neighbors[1].0 + neighbors[2].0 - x * 3;
            let ay = neighbors[0].1 + neighbors[1].1 + neighbors[2].1 - y * 3;
            let (n1x, n1y, n2x, n2y) = if ax == 0 {
                (x - 1, y + ay, x + 1, y + ay)
            } else {
                (x + ax, y - 1, x + ax, y + 1)
            };
            [(n1x, n1y), (n2x, n2y)]
                .iter()
                .filter(|(nx, ny)| {
                    garden[*ny as usize][*nx as usize] != garden[y as usize][x as usize]
                })
                .count()
        }
        4 => [
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x + 1, y - 1),
            (x + 1, y + 1),
        ]
        .iter()
        .filter(|(nx, ny)| garden[*ny as usize][*nx as usize] != garden[y as usize][x as usize])
        .count(),
        _ => panic!("Invalid number of neighbors"),
    };
    for (nx, ny) in get_valid_neighbors(x, y, garden) {
        if visited[ny as usize][nx as usize] {
            continue;
        }
        let (n_area, n_perimeter) = flood_fill(nx, ny, width, height, garden, visited);
        area += n_area;
        perimeter += n_perimeter;
    }
    return (area, perimeter);
}

fn main() {
    let garden = parse_input();
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];
    let (width, height) = (garden[0].len(), garden.len());
    let mut price = 0;
    for y in 0..height {
        for x in 0..width {
            if !visited[y][x] {
                let (area, perimeter) =
                    flood_fill(x as i32, y as i32, width, height, &garden, &mut visited);
                price += area * perimeter;
            }
        }
    }
    println!("{:?}", price);
}
