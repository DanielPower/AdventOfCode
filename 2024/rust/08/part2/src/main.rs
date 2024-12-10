use std::collections::{HashMap, HashSet};

fn parse_input() -> (HashMap<char, Vec<(i32, i32)>>, i32, i32) {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in std::io::stdin().lines().enumerate() {
        let line = line.unwrap();
        height += 1;
        if width == 0 {
            width = line.len() as i32;
        }
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => continue,
                _ => {
                    antennas
                        .entry(ch)
                        .or_insert(Vec::new())
                        .push((x as i32, y as i32));
                }
            }
        }
    }
    (antennas, width, height)
}

fn pairs<T>(v: &[T]) -> impl Iterator<Item = (&T, &T)> {
    v.iter().enumerate().flat_map(move |(i, a)| {
        v.iter()
            .enumerate()
            .filter(move |(j, _)| i != *j)
            .map(move |(_, b)| (a, b))
    })
}

fn main() {
    let (antennas, width, height) = parse_input();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_, signal) in antennas {
        for (a, b) in pairs(&signal) {
            let diff = (a.0 - b.0, a.1 - b.1);
            let mut i = 0;
            loop {
                let antinode = (a.0 + diff.0 * i, a.1 + diff.1 * i);
                if antinode.0 < 0 || antinode.0 >= width || antinode.1 < 0 || antinode.1 >= height {
                    break;
                }
                antinodes.insert(antinode);
                i += 1;
            }
        }
    }
    println!("{}", antinodes.len());
}
