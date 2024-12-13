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

fn get_neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < width - 1 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < height - 1 {
        neighbors.push((x, y + 1));
    }
    neighbors
}

fn flood_fill(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    garden: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> (i32, i32) {
    let mut perimeter = 4;
    let mut area = 1;
    visited[y][x] = true;
    for (nx, ny) in get_neighbors(x, y, width, height) {
        if garden[y][x] == garden[ny][nx] {
            perimeter -= 1;
            if visited[ny][nx] {
                continue;
            }
            let (n_area, n_perimeter) = flood_fill(nx, ny, width, height, garden, visited);
            area += n_area;
            perimeter += n_perimeter;
        }
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
                let (area, perimeter) = flood_fill(x, y, width, height, &garden, &mut visited);
                price += area * perimeter;
            }
        }
    }
    println!("{:?}", price,);
}
