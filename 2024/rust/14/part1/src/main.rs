use regex;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const TIME: i32 = 100;

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

fn main() {
    let robots = parse_input();
    let moved_robots = robots
        .iter()
        .map(|&(px, py, vx, vy)| position(px, py, vx, vy, TIME))
        .collect::<Vec<_>>();
    let mut quadrants: Vec<i32> = vec![0, 0, 0, 0];
    for (x, y) in moved_robots {
        if x < WIDTH / 2 && y < HEIGHT / 2 {
            quadrants[0] += 1;
        } else if x > WIDTH / 2 && y < HEIGHT / 2 {
            quadrants[1] += 1;
        } else if x < WIDTH / 2 && y > HEIGHT / 2 {
            quadrants[2] += 1;
        } else if x > WIDTH / 2 && y > HEIGHT / 2 {
            quadrants[3] += 1;
        }
    }
    println!(
        "{}",
        quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
    );
}
