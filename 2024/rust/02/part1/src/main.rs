fn main() {
    let mut safe = 0;
    'outer: for line in std::io::stdin().lines() {
        let levels: Vec<i32> = line
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut increasing: Option<bool> = None;
        for pair in levels.windows(2) {
            let [a, b] = pair else { panic!() };
            match increasing {
                None => {
                    increasing = Some(a < b);
                }
                Some(true) => {
                    if a > b {
                        continue 'outer;
                    }
                }
                Some(false) => {
                    if a < b {
                        continue 'outer;
                    }
                }
            }
            let difference = a - b;
            if difference.abs() > 3 || a == b {
                continue 'outer;
            }
        }
        safe += 1;
    }
    println!("{}", safe);
}
