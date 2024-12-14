use regex;

const OFFSET: i64 = 10000000000000;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Game {
    a: Point,
    b: Point,
    prize: Point,
}

fn parse_input() -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    let re_button = regex::Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let re_prize = regex::Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
    let mut iter = std::io::stdin().lines();
    loop {
        let line = iter.next().unwrap().unwrap();
        let a = re_button.captures(&line).unwrap();
        let ax = a.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let ay = a.get(2).unwrap().as_str().parse::<i64>().unwrap();

        let line = iter.next().unwrap().unwrap();
        let b = re_button.captures(&line).unwrap();
        let bx = b.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let by = b.get(2).unwrap().as_str().parse::<i64>().unwrap();

        let line = iter.next().unwrap().unwrap();
        let prize = re_prize.captures(&line).unwrap();
        let px = prize.get(1).unwrap().as_str().parse::<i64>().unwrap() + OFFSET;
        let py = prize.get(2).unwrap().as_str().parse::<i64>().unwrap() + OFFSET;

        games.push(Game {
            a: Point { x: ax, y: ay },
            b: Point { x: bx, y: by },
            prize: Point { x: px, y: py },
        });

        let line = iter.next();
        if matches!(line, None) {
            break;
        }
    }
    games
}

fn check_game(game: &Game, n: i64, m: i64) -> bool {
    if n < 0 || m < 0 {
        return false;
    }
    let x = game.a.x * n + game.b.x * m;
    let y = game.a.y * n + game.b.y * m;
    x == game.prize.x && y == game.prize.y
}

fn solve_game(game: &Game) -> Option<(i64, i64)> {
    let determinant = game.a.x * game.b.y - game.a.y * game.b.x;
    if determinant == 0 {
        return None;
    }

    let n = (game.prize.x * game.b.y - game.prize.y * game.b.x) / determinant;
    let m = (game.a.x * game.prize.y - game.a.y * game.prize.x) / determinant;

    if check_game(game, n, m) {
        Some((n, m))
    } else {
        None
    }
}

fn main() {
    let games = parse_input();
    let mut total = 0;
    for game in games {
        match solve_game(&game) {
            Some((a, b)) => total += 3 * a + b,
            None => (),
        }
    }
    println!("Total: {}", total);
}
