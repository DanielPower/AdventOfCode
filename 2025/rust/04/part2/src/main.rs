use std::io::Read;

fn main() {
    let mut total = 0;
    let mut buffer = String::new();
    std::io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut lines: Vec<Vec<char>> = buffer
        .split_whitespace()
        .map(|line| line.chars().collect())
        .collect();
    let mut dirty = true;
    while dirty {
        dirty = false;
        for y in 0..lines.len() {
            for x in 0..lines[0].len() {
                if lines[y][x] != '@' {
                    continue;
                }
                let neighbors: isize = [
                    [-1_isize, -1_isize],
                    [0_isize, -1_isize],
                    [1_isize, -1_isize],
                    [-1_isize, 0_isize],
                    [1_isize, 0_isize],
                    [-1_isize, 1_isize],
                    [0_isize, 1_isize],
                    [1_isize, 1_isize],
                ]
                .iter()
                .map(|[dx, dy]| {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx < 0
                        || ny < 0
                        || ny >= lines.len() as isize
                        || nx >= lines[0].len() as isize
                    {
                        return 0;
                    }
                    if lines[ny as usize][nx as usize] == '@' {
                        1
                    } else {
                        0
                    }
                })
                .sum();
                if neighbors < 4 {
                    lines[y][x] = 'x';
                    dirty = true;
                    total += 1;
                }
            }
        }
    }
    println!("{}", total);
}
