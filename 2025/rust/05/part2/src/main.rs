fn overlaps(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

fn main() {
    let mut fresh_ranges: Vec<(i64, i64)> = Vec::new();
    for line in std::io::stdin().lines().map(Result::unwrap) {
        if line.is_empty() {
            break;
        }
        let (start, end) = line.split_once('-').unwrap();
        let (start, end): (i64, i64) = (start.parse().unwrap(), end.parse().unwrap());

        let overlap_ranges: Vec<(usize, (i64, i64))> = fresh_ranges
            .iter()
            .enumerate()
            .filter_map(|(i, other)| match overlaps((start, end), *other) {
                true => Some((i, *other)),
                false => None,
            })
            .collect();

        if overlap_ranges.is_empty() {
            fresh_ranges.push((start, end));
            fresh_ranges.sort();
        } else {
            let (index1, (start1, _end1)) = overlap_ranges.first().unwrap();
            let (index2, (_start2, end2)) = overlap_ranges.last().unwrap();
            let new_start = start.min(*start1);
            let new_end = end.max(*end2);
            fresh_ranges[*index1] = (new_start, new_end);
            if index1 != index2 {
                fresh_ranges.drain(*index1 + 1..=*index2);
            }
        }
    }
    let mut total_fresh: i64 = 0;
    for (start, end) in &fresh_ranges {
        total_fresh += end - start + 1;
    }
    println!("{}", total_fresh);
}
