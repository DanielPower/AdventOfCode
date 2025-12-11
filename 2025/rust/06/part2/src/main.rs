fn main() {
    let mut number_strings: Vec<String> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    let first_line = std::io::stdin().lines().next().unwrap().unwrap();
    for c in first_line.chars() {
        number_strings.push(if c == ' ' {
            String::new()
        } else {
            String::from(c)
        });
    }
    for line in std::io::stdin().lines().map(Result::unwrap) {
        if line.contains("+") || line.contains("*") {
            operators = line
                .split_whitespace()
                .map(|c| c.chars().next().unwrap())
                .collect();
            break;
        }
        for (i, c) in line.chars().enumerate() {
            if c != ' ' {
                number_strings[i].push(c);
            }
        }
    }

    let mut operation = 0;
    let mut problems: Vec<(char, Vec<i64>)> = Vec::new();
    for operator in operators {
        problems.push((operator, Vec::new()));
    }
    for number_string in number_strings {
        if number_string.is_empty() {
            operation += 1;
            continue;
        }
        problems[operation].1.push(number_string.parse().unwrap());
    }

    let mut total = 0;
    for problem in problems {
        let (operator, numbers) = problem;
        let result = numbers
            .into_iter()
            .reduce(|n, m| match operator {
                '+' => n + m,
                '*' => n * m,
                _ => panic!("I like your funny words, magic man"),
            })
            .unwrap();
        total += result;
    }
    println!("{}", total);
}
