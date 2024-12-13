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
    regions: &mut Vec<Vec<(i32, i32)>>,
) {
    visited[y][x] = true;
    regions.last_mut().unwrap().push((x as i32, y as i32));
    let neighbors = get_neighbors(x, y, width, height);
    for (nx, ny) in neighbors {
        if garden[y][x] == garden[ny][nx] {
            if visited[ny][nx] {
                continue;
            }
            flood_fill(nx, ny, width, height, garden, visited, regions);
        }
    }
}

fn count_walls(region: &Vec<(i32, i32)>) -> i32 {
    let hash_by_x: std::collections::HashMap<i32, Vec<i32>> =
        region
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, (x, y)| {
                acc.entry(*x).or_insert(Vec::new());
                acc.get_mut(x).unwrap().push(*y);
                acc
            });
    let (min_x, max_x) = hash_by_x.iter().fold((i32::MAX, i32::MIN), |acc, (x, _)| {
        (acc.0.min(*x), acc.1.max(*x))
    });
    let mut wall_count = 0;
    for x in hash_by_x.keys() {
        let mut y_values = hash_by_x.get(x).unwrap().clone();
        y_values.sort();
        let y_diffs: Vec<i32> = y_values
            .windows(2)
            .map(|w| if (w[1] - w[0]).abs() > 1 { 1 } else { 0 })
            .collect();
        println!("x: {}, y_diffs: {:?}", x, y_diffs);
    }
    wall_count
}

fn main() {
    let garden = parse_input();
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];
    let (width, height) = (garden[0].len(), garden.len());
    let mut regions: Vec<Vec<(i32, i32)>> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if !visited[y][x] {
                regions.push(Vec::new());
                flood_fill(x, y, width, height, &garden, &mut visited, &mut regions);
            }
        }
    }
    let mut wall_count = 0;
    //for region in &regions {
    //    wall_count += count_walls(region);
    //}
    wall_count = count_walls(&regions[0]);
    println!("Number of walls: {}", wall_count);
}
