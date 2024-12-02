fn main() {
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut parts = line.trim().split_whitespace();
        a.push(parts.next().unwrap().parse().unwrap());
        b.push(parts.next().unwrap().parse().unwrap());
    }
    a.sort();
    b.sort();
    let mut differences = 0;
    for i in 0..a.len() {
        differences += (a[i] - b[i]).abs();
    }
    println!("{}", differences);
}
