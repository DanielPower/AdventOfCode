fn parse_input() -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }
    grid
}

fn get_neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn travel_trailhead(grid: &Vec<Vec<i32>>, w: i32, h: i32, x: i32, y: i32) -> i32 {
    let value = grid[y as usize][x as usize];
    if value == 9 {
        return 1;
    }
    let neighbors = get_neighbors(x, y).iter().fold(0, |acc, (nx, ny)| {
        if *nx >= 0
            && *ny >= 0
            && *nx < w
            && *ny < h
            && grid[*ny as usize][*nx as usize] == value + 1
        {
            acc + travel_trailhead(grid, w, h, *nx, *ny)
        } else {
            acc
        }
    });
    neighbors
}

fn main() {
    let grid = parse_input();
    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                let value = travel_trailhead(
                    &grid,
                    grid[y].len() as i32,
                    grid.len() as i32,
                    x as i32,
                    y as i32,
                );
                println!("({}, {}): {}", x, y, value);
                total += value;
            }
        }
    }
    println!("{}", total);
}
