enum SafeStepResult {
    Safe,
    Unsafe(usize),
}

fn is_safe(levels: Vec<i32>) -> SafeStepResult {
    let direction = levels[0] - levels[1];
    for i in 0..levels.len() - 1 {
        let a = levels[i];
        let b = levels[i + 1];
        let difference = a - b;
        if difference.abs() > 3 || difference == 0 || direction * difference < 0 {
            return SafeStepResult::Unsafe(i);
        }
    }
    return SafeStepResult::Safe;
}

fn main() {
    let mut safe = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }
        let levels: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        match is_safe(levels.clone()) {
            SafeStepResult::Unsafe(i) => {
                let mut levels_a = levels.clone();
                let mut levels_b = levels.clone();
                let mut levels_c = levels.clone();
                levels_a.remove(i.try_into().unwrap());
                levels_b.remove((i + 1).try_into().unwrap());
                levels_c.remove((0).try_into().unwrap());
                if matches!(is_safe(levels_a), SafeStepResult::Safe)
                    || matches!(is_safe(levels_b), SafeStepResult::Safe)
                    || matches!(is_safe(levels_c), SafeStepResult::Safe)
                {
                    safe += 1;
                }
            }
            SafeStepResult::Safe => {
                safe += 1;
            }
        }
    }
    println!("{}", safe);
}
