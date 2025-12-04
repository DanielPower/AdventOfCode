fn middle_split_index(n: i64) -> i64 {
    if n < 10 { 0 }
    else if n < 100 { 1 }
    else if n < 1_000 { 0 }
    else if n < 10_000 { 2 }
    else if n < 100_000 { 0 }
    else if n < 1_000_000 { 3 }
    else if n < 10_000_000 { 0 }
    else if n < 100_000_000 { 4 }
    else if n < 1_000_000_000 { 0 }
    else if n < 10_000_000_000 { 5 }
    else if n < 100_000_000_000 { 0 }
    else if n < 1_000_000_000_000 { 6 }
    else if n < 10_000_000_000_000 { 0 }
    else if n < 100_000_000_000_000 { 7 }
    else if n < 1_000_000_000_000_000 { 0 }
    else if n < 10_000_000_000_000_000 { 8 }
    else if n < 100_000_000_000_000_000 { 0 }
    else if n < 1_000_000_000_000_000_000 { 9 }
    else { 0 }
}

fn main() {
    let mut line = String::new();
    let mut total = 0;
    std::io::stdin().read_line(&mut line).unwrap();
    for range in line.trim().split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();
        for i in start..=end {
            let split_index = middle_split_index(i);
            if split_index != 0 {
                let i_string = i.to_string();
                let (first, second) = i_string.split_at(split_index as usize);
                if first == second {
                    total += i;
                }
            }
        }
    }
    println!("{}", total);
}
