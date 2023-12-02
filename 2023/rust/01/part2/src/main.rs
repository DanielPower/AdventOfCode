use std::io;

fn first_digit(line: &String) -> char {
    if line.is_empty() {
        panic!("Empty line");
    }
    if line.starts_with("1") || line.starts_with("one") {
        '1'
    } else if line.starts_with("2") || line.starts_with("two") {
        '2'
    } else if line.starts_with("3") || line.starts_with("three") {
        '3'
    } else if line.starts_with("4") || line.starts_with("four") {
        '4'
    } else if line.starts_with("5") || line.starts_with("five") {
        '5'
    } else if line.starts_with("6") || line.starts_with("six") {
        '6'
    } else if line.starts_with("7") || line.starts_with("seven") {
        '7'
    } else if line.starts_with("8") || line.starts_with("eight") {
        '8'
    } else if line.starts_with("9") || line.starts_with("nine") {
        '9'
    } else {
        let mut chars = line.chars();
        chars.next();
        first_digit(&chars.as_str().to_string())
    }
}

fn last_digit(line: &String) -> char {
    if line.starts_with("1") || line.starts_with("eno") {
        '1'
    } else if line.starts_with("2") || line.starts_with("owt") {
        '2'
    } else if line.starts_with("3") || line.starts_with("eerht") {
        '3'
    } else if line.starts_with("4") || line.starts_with("ruof") {
        '4'
    } else if line.starts_with("5") || line.starts_with("evif") {
        '5'
    } else if line.starts_with("6") || line.starts_with("xis") {
        '6'
    } else if line.starts_with("7") || line.starts_with("neves") {
        '7'
    } else if line.starts_with("8") || line.starts_with("thgie") {
        '8'
    } else if line.starts_with("9") || line.starts_with("enin") {
        '9'
    } else {
        let mut chars = line.chars();
        chars.next();
        last_digit(&chars.as_str().to_string())
    }
}

fn main() -> io::Result<()> {
    let mut numbers: Vec<i64> = Vec::new();
    for line in io::stdin().lines() {
        let line = line?;
        println!("{}", line);
        let a = first_digit(&line);
        let reversed_line: String = line.chars().rev().collect();
        let b = last_digit(&reversed_line);
        numbers.push(format!("{}{}", a, b).parse::<i64>().unwrap());
    }
    println!("{}", numbers.iter().sum::<i64>());
    Ok(())
}
