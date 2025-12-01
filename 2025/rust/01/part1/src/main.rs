fn main() {
    let mut zero_count: i32 = 0;
    let mut accumulator: i32 = 50;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (direction, magnitude) = line.trim().split_at(1);
        let magnitude: i32 = magnitude.parse().unwrap();
        accumulator += if direction == "R" {magnitude} else {-magnitude};
        accumulator = accumulator.rem_euclid(100);
        if accumulator == 0 {
            zero_count += 1;
        }
        println!("Direction: {}, Magnitude: {}, Accumulator: {}", direction, magnitude, accumulator);
    }
    println!("{}", zero_count);
}
