use std::collections::HashMap;

fn parse_input() -> HashMap<i64, i64> {
    let stones_vec = std::io::stdin().lines().next().unwrap().unwrap();
    let stones_vec: Vec<i64> = stones_vec
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut stones = HashMap::new();
    for stone in stones_vec {
        stones.insert(stone, stones.get(&stone).unwrap_or(&0) + 1);
    }
    stones
}

fn blink(input: &HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut output = HashMap::new();
    for (stone, count) in input {
        if *stone == 0 {
            output.insert(1, output.get(&1).unwrap_or(&0) + count);
        } else {
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let (a, b): (&str, &str) = stone_str.split_at(stone_str.len() / 2);
                let (a, b): (i64, i64) = (a.parse().unwrap(), b.parse().unwrap());
                output.insert(a, output.get(&a).unwrap_or(&0) + count);
                output.insert(b, output.get(&b).unwrap_or(&0) + count);
            } else {
                let ohfour = stone * 2024;
                output.insert(ohfour, output.get(&(ohfour)).unwrap_or(&0) + count);
            }
        }
    }
    output
}

fn main() {
    let mut stones = parse_input();
    for _ in 0..75 {
        stones = blink(&stones);
    }
    println!("{:?}", stones.iter().fold(0, |acc, (_, count)| acc + count));
}
