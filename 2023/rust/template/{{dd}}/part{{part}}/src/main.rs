// {{year}} day {{day}}, part {{part}}: https://adventofcode.com/{{year}}/day/{{day}}
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("reading stdin");
    println!("{}", solve(&input));
}

fn solve(input: &str) -> usize {
    input.lines().count()
}
