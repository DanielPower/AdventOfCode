use std::collections::HashSet;

fn get_factors(n: i64) -> Vec<usize> {
    if n < 10 { vec![] }
    else if n < 1_000 { vec![1] }
    else if n < 10_000 { vec![1, 2] }
    else if n < 100_000 { vec![1] }
    else if n < 1_000_000 { vec![1, 2, 3] }
    else if n < 10_000_000 { vec![1] }
    else if n < 100_000_000 { vec![1, 2, 4] }
    else if n < 1_000_000_000 { vec![1, 3] }
    else if n < 10_000_000_000 { vec![1, 2, 5] }
    else if n < 100_000_000_000 { vec![1] }
    else if n < 1_000_000_000_000 { vec![1, 2, 3, 4, 6] }
    else if n < 10_000_000_000_000 { vec![1] }
    else if n < 100_000_000_000_000 { vec![1, 2, 7] }
    else if n < 1_000_000_000_000_000 { vec![1, 3, 5] }
    else if n < 10_000_000_000_000_000 { vec![1, 2, 8] }
    else if n < 100_000_000_000_000_000 { vec![1] }
    else if n < 1_000_000_000_000_000_000 { vec![1, 2, 6, 9] }
    else { vec![] }
}

fn is_valid(i: i64) -> bool {
    let chars: Vec<char> = i.to_string().chars().collect();
    let mut valid = false;
    let factors = get_factors(i);
    if factors.is_empty() {
        return true;
    }
    for split_index in get_factors(i) {
        valid = false;
        let mut chunks = chars.chunks(split_index);
        let first_chunk = chunks.next().unwrap();
        'outer: for chunk in chunks {
            for (index, char) in chunk.iter().enumerate() {
                if *char != first_chunk[index] {
                    valid = true;
                    break 'outer;
                }
            }
        }
        if !valid {
            return false;
        }
    }
    valid
}

fn main() {
    let mut line = String::new();
    let mut invalid_ids: HashSet<i64> = HashSet::new();
    std::io::stdin().read_line(&mut line).unwrap();
    for range in line.trim().split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();
        for i in start..=end {
            if !is_valid(i) {
                invalid_ids.insert(i);
            }
        }
    }
    let total: i64 = invalid_ids.iter().sum();
    println!("{}", total);
}
