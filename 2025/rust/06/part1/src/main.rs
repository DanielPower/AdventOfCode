fn parse_numbers(line: &str) -> Option<Vec<i64>> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().ok())
        .collect()
}

fn main() {
    let mut number_columns: Vec<Vec<i64>> = Vec::new();
    let numbers = parse_numbers(&std::io::stdin().lines().next().unwrap().unwrap()).unwrap();
    let mut operators: Vec<char> = Vec::new();
    for number in numbers {
        number_columns.push(Vec::from([number]));
    }
    for line in std::io::stdin().lines().map(Result::unwrap) {
        if let Some(numbers) = parse_numbers(&line) {
            for (index, number) in numbers.iter().enumerate() {
                number_columns[index].push(*number);
            }
        } else {
            operators = line
                .split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .collect();
        }
    }

    let mut total = 0;
    for (index, numbers) in number_columns.iter().enumerate() {
        let operator = &operators[index];
        let value = numbers
            .iter()
            .copied()
            .reduce(|x, y| match operator {
                '*' => x * y,
                '+' => x + y,
                _ => panic!("I like your funny words, magic man"),
            })
            .unwrap();
        total += value;
    }
    println!("{}", total);
}
