use std::collections::{HashMap, HashSet};

fn shoot(
    rows: &[HashSet<usize>],
    position: usize,
    depth: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    match cache.get(&(position, depth)) {
        Some(result) => *result,
        None => {
            let result = match rows.split_first() {
                Some((row, rest)) => {
                    if row.contains(&position) {
                        return shoot(rest, position - 1, depth + 1, cache)
                            + shoot(rest, position + 1, depth + 1, cache);
                    }
                    shoot(rest, position, depth + 1, cache)
                }
                None => 1,
            };
            cache.insert((position, depth), result);
            result
        }
    }
}

fn main() {
    let mut rows: Vec<HashSet<usize>> = Vec::new();

    let start_position = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap();

    std::io::stdin().lines().next();
    for (index, line) in std::io::stdin().lines().map(Result::unwrap).enumerate() {
        if index % 2 == 1 {
            continue;
        }
        let mut splitters: HashSet<usize> = HashSet::new();
        for (index, char) in line.chars().enumerate() {
            if char == '^' {
                splitters.insert(index);
            }
        }
        rows.push(splitters);
    }

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    println!("{}", shoot(&rows[..], start_position, 0, &mut cache));
}
