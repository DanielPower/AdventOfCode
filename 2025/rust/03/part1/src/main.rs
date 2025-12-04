fn main() {
    let mut total = 0;
    for line in std::io::stdin().lines() {
        let batteries: Vec<u32> = line
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let mut max_value = 0;
        let mut max_index = 0;
        for (index, battery) in batteries[..batteries.len() - 1].iter().enumerate() {
            if *battery > max_value {
                max_value = *battery;
                max_index = index;
            }
        }
        let mut snd_value = 0;
        for battery in &batteries[max_index + 1..] {
            if *battery > snd_value {
                snd_value = *battery;
            }
        }
        total += max_value * 10 + snd_value;
    }
    println!("{}", total);
}
