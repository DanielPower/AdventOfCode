use regex::Regex;
use std::io::Read;

fn main() {
    let mut enabled = true;
    let mut total = 0;

    let mut buffer = String::new();
    std::io::stdin().lock().read_to_string(&mut buffer).unwrap();

    let re =
        Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|(?<mul>mul\((?<mul_a>\d+),(?<mul_b>\d+)\))")
            .unwrap();
    for caps in re.captures_iter(buffer.as_str()) {
        if let Some(_) = caps.name("do") {
            enabled = true
        }
        if let Some(_) = caps.name("dont") {
            enabled = false;
        }
        if let Some(_) = caps.name("mul") {
            if enabled {
                let a = caps.name("mul_a").unwrap().as_str().parse::<i32>().unwrap();
                let b = caps.name("mul_b").unwrap().as_str().parse::<i32>().unwrap();
                total += a * b;
            }
        }
    }
    println!("{}", total);
}
