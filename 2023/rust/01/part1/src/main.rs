use regex::Regex;
use std::io;

fn main() -> io::Result<()> {
    let re_first_digit = Regex::new(r"^[^\d]*(\d)").unwrap();
    let re_last_digit = Regex::new(r"(\d)[^\d]*$").unwrap();
    let mut numbers: Vec<i64> = Vec::new();
    for line in io::stdin().lines() {
        let line = line?;
        let first_digit = re_first_digit
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let last_digit = re_last_digit
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let concatenated = format!("{}{}", first_digit, last_digit);
        numbers.push(concatenated.parse::<i64>().unwrap());
    }
    println!("{}", numbers.iter().sum::<i64>());
    Ok(())
}
