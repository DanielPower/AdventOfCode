use std::collections::HashSet;

use regex;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn parse_input() -> Vec<(i32, i32, i32, i32)> {
    let re = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<(i32, i32, i32, i32)> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();
        let px = caps[1].parse::<i32>().unwrap();
        let py = caps[2].parse::<i32>().unwrap();
        let vx = caps[3].parse::<i32>().unwrap();
        let vy = caps[4].parse::<i32>().unwrap();
        robots.push((px, py, vx, vy));
    }
    robots
}

fn position(px: i32, py: i32, vx: i32, vy: i32, t: i32) -> (i32, i32) {
    (
        (px + vx * t).rem_euclid(WIDTH),
        (py + vy * t).rem_euclid(HEIGHT),
    )
}

// If there's a 3x3 cluster of robots, there's a good potential for a tree
fn has_cluster(robots: &HashSet<(i32, i32)>) -> bool {
    for x in 0..WIDTH {
        'outer: for y in 0..HEIGHT {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if !robots.contains(&(x + dx, y + dy)) {
                        continue 'outer;
                    }
                }
            }
            return true;
        }
    }
    return false;
}

fn main() {
    let robots = parse_input();
    for t in 0..10000 {
        let moved_robots = robots
            .iter()
            .map(|&(px, py, vx, vy)| position(px, py, vx, vy, t))
            .collect::<HashSet<(i32, i32)>>();

        if has_cluster(&moved_robots) {
            println!("t: {}", t);
        }
    }
}
