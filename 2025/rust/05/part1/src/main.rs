fn main() {
    let mut fresh_ranges: Vec<(i64, i64)> = Vec::new();
    let mut fresh_ingredients = 0;
    for line in std::io::stdin().lines().map(Result::unwrap) {
        println!("{}", line);
        if line.is_empty() {
            break;
        }
        let (first, second) = line.split_once('-').unwrap();
        fresh_ranges.push((first.parse().unwrap(), second.parse().unwrap()));
    }
    for line in std::io::stdin().lines().map(Result::unwrap) {
        let ingredient_id: i64 = line.parse().unwrap();
        for (start, end) in &fresh_ranges {
            if ingredient_id >= *start && ingredient_id <= *end {
                fresh_ingredients += 1;
                break;
            }
        }
    }
    println!("{}", fresh_ingredients);
}
