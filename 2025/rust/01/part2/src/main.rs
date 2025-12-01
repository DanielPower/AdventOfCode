fn main() {
    let mut zero_count: i32 = 0;
    let mut accumulator: i32 = 50;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (direction, magnitude) = line.trim().split_at(1);
        let magnitude: i32 = magnitude.parse().unwrap();
        let delta = if direction == "R" {magnitude} else {-magnitude};
        if accumulator != 0 && delta < -accumulator {
            zero_count += 1;
        }
        accumulator += delta;
        zero_count += (accumulator / 100).abs();
        if accumulator == 0 {
            zero_count += 1;
        }
        accumulator = accumulator.rem_euclid(100);
    }
    println!("{}", zero_count);
}
