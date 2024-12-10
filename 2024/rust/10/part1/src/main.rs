use std::collections::HashSet;

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

fn travel_trailhead(
    grid: &Vec<Vec<i32>>,
    peaks: &mut HashSet<(i32, i32)>,
    w: i32,
    h: i32,
    x: i32,
    y: i32,
) {
    let value = grid[y as usize][x as usize];
    if value == 9 {
        peaks.insert((x, y));
    }
    for (nx, ny) in vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        if nx >= 0 && ny >= 0 && nx < w && ny < h && grid[ny as usize][nx as usize] == value + 1 {
            travel_trailhead(grid, peaks, w, h, nx, ny);
        }
    }
}

fn main() {
    let grid = parse_input();
    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                let mut peaks: HashSet<(i32, i32)> = HashSet::new();
                travel_trailhead(
                    &grid,
                    &mut peaks,
                    grid[y].len() as i32,
                    grid.len() as i32,
                    x as i32,
                    y as i32,
                );
                total += peaks.len();
            }
        }
    }
    println!("{}", total);
}
