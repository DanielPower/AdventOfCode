use std::io::BufRead;

// This floating point trickery runs twice as fast as string concatenation
fn concatenate(a: usize, b: usize) -> usize {
    let b_digits = (b as f64).log10().floor() as u32 + 1; // Count the digits in b
    a * 10usize.pow(b_digits) + b
}

fn evaluate(acc: usize, mut values: Vec<usize>, result: usize) -> bool {
    if values.is_empty() {
        return acc == result;
    }
    let value = values.pop().unwrap();
    evaluate(acc + value, values.clone(), result)
        || evaluate(acc * value, values.clone(), result)
        || evaluate(concatenate(acc, value), values.clone(), result)
}

fn main() {
    let mut total = 0;
    let re = regex::Regex::new(r"^(?<result>\d+): (?<values>\d+(?: \d+)*)").unwrap();
    for line in std::io::stdin().lock().lines() {
        for cap in re.captures_iter(&line.unwrap()) {
            let result = cap
                .name("result")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let values = cap
                .name("values")
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .rev()
                .collect::<Vec<_>>();
            if evaluate(0, values, result) {
                total += result;
            }
        }
    }
    println!("Total: {}", total);
}
