fn blink(input: &Vec<i64>) -> Vec<i64> {
    let mut output = Vec::new();
    for stone in input {
        match stone {
            0 => output.push(1),
            i => {
                let i_str = i.to_string();
                if i_str.len() % 2 == 0 {
                    let (a, b): (&str, &str) = i_str.split_at(i.to_string().len() / 2);
                    let a: i64 = a.parse().unwrap();
                    let b: i64 = b.parse().unwrap();
                    output.push(a);
                    output.push(b);
                } else {
                    output.push(i * 2024);
                }
            }
        }
    }
    output
}

fn main() {
    let stones = std::io::stdin().lines().next().unwrap().unwrap();
    let mut stones: Vec<i64> = stones
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    for _ in 0..25 {
        stones = blink(&stones);
    }
    println!("{:?}", stones.len());
}
