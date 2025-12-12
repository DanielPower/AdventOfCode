use std::collections::HashSet;

fn main() {
    let mut rows: Vec<Vec<i32>> = Vec::new();
    std::io::stdin().lines().next();
    std::io::stdin().lines().next();
    for (index, line) in std::io::stdin().lines().map(Result::unwrap).enumerate() {
        if index % 2 == 1 {
            continue;
        }
        let mut row_positions: Vec<i32> = Vec::new();
        for (index, char) in line.chars().enumerate() {
            if char == '^' {
                row_positions.push(index as i32);
            }
        }
        rows.push(row_positions);
    }
    let mut splits = 1; // Assume the first beam will be split, or else the input is silly
    let start = rows[0][0];
    let mut beams: HashSet<i32> = HashSet::from([start - 1, start + 1]);
    for row in rows.iter().skip(1) {
        for position in row {
            if beams.contains(position) {
                beams.remove(position);
                beams.insert(position - 1);
                beams.insert(position + 1);
                splits += 1
            }
        }
    }
    println!("{}", splits);
}
