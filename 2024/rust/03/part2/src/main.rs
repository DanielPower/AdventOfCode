use regex::Regex;
use std::io::Read;

const DO: usize = 1;
const DONT: usize = 2;
const MUL: usize = 3;
const MUL_A: usize = 4;
const MUL_B: usize = 5;

fn main() {
    let mut enabled = true;
    let mut total = 0;

    let mut buffer = String::new();
    std::io::stdin().lock().read_to_string(&mut buffer).unwrap();

    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\((\d+),(\d+)\))").unwrap();
    for foo in re.captures_iter(buffer.as_str()) {
        if let Some(_) = foo.get(DO) {
            enabled = true;
        }
        if let Some(_) = foo.get(DONT) {
            enabled = false;
        }
        if let Some(_) = foo.get(MUL) {
            if enabled {
                let a = foo.get(MUL_A).unwrap().as_str().parse::<i32>().unwrap();
                let b = foo.get(MUL_B).unwrap().as_str().parse::<i32>().unwrap();
                total += a * b;
            }
        }
    }
    println!("{}", total);
}
