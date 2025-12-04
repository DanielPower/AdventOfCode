fn take_best_battery(bank: &[i64]) -> (i64, usize) {
    let mut max_value = 0;
    let mut max_index = 0;
    for (index, battery) in bank.iter().enumerate() {
        if *battery > max_value {
            max_value = *battery;
            max_index = index;
        }
    }
    (max_value, max_index)
}

fn batteries_to_number(batteries: &[i64]) -> i64 {
    let mut total: i64 = 0;
    batteries.iter().enumerate().for_each(|(i, battery)| {
        total += battery * 10_i64.pow((batteries.len() - 1 - i) as u32);
    });
    total
}

fn main() {
    let mut total: i64 = 0;
    for line in std::io::stdin().lines() {
        let mut best_batteries: Vec<i64> = Vec::new();
        let batteries: Vec<i64> = line
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();

        let mut index = 0;
        for i in (0..12).rev() {
            let (value, new_index) = take_best_battery(&batteries[index..batteries.len() - i]);
            index += new_index + 1;
            best_batteries.push(value);
        }
        total += batteries_to_number(&best_batteries);
    }
    println!("{}", total);
}
