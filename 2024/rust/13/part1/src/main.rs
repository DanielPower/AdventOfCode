use regex;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
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
        let ax = a.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let ay = a.get(2).unwrap().as_str().parse::<i32>().unwrap();

        let line = iter.next().unwrap().unwrap();
        let b = re_button.captures(&line).unwrap();
        let bx = b.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let by = b.get(2).unwrap().as_str().parse::<i32>().unwrap();

        let line = iter.next().unwrap().unwrap();
        let prize = re_prize.captures(&line).unwrap();
        let px = prize.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let py = prize.get(2).unwrap().as_str().parse::<i32>().unwrap();

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

fn check_game(game: &Game, a: i32, b: i32) -> bool {
    let x = game.a.x * a + game.b.x * b;
    let y = game.a.y * a + game.b.y * b;
    x == game.prize.x && y == game.prize.y
}

fn solve_game(game: &Game) -> Option<(i32, i32)> {
    let initial_a = game.prize.x / game.a.x;
    let mut minimum_presses = i32::MAX;
    let mut best_solution: Option<(i32, i32)> = None;
    for a in (0..initial_a).rev() {
        let b = (game.prize.x - game.a.x * a) / game.b.x;
        if a + b < minimum_presses && check_game(game, a, b) {
            minimum_presses = a + b;
            best_solution = Some((a, b));
        }
    }
    best_solution
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
