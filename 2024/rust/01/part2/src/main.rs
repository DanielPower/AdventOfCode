use std::collections::HashMap;

fn main() {
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut parts = line.trim().split_whitespace();
        a.push(parts.next().unwrap().parse().unwrap());
        b.push(parts.next().unwrap().parse().unwrap());
    }
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for i in 0..b.len() {
        *counts.entry(b[i]).or_insert(0) += 1;
    }
    let mut similarities = 0;
    for i in 0..a.len() {
        similarities += a[i] * counts.get(&a[i]).unwrap_or(&0);
    }
    println!("{}", similarities);
}
