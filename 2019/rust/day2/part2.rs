use std::fs;
use std::string::String;

fn read_instructions(filename: String) -> Vec<u32> {
    let mut values = Vec::new();
    {
        let input = fs::read_to_string(String::from(filename))
            .expect("Something went wrong reading the file");
        let mut string_value = String::from("");
        for character in input.chars() {
            if character == ',' {
                values.push(string_value.parse::<u32>().unwrap());
                string_value = String::from("");
            } else if character >= '0' && character <= '9' {
                string_value.push(character);
            }
        }
        values.push(string_value.parse::<u32>().unwrap());
    }
    return values;
}

fn run(mut values: Vec<u32>) -> Vec<u32> {
    let mut ip = 0;
    loop {
        let opcode = values[ip];
        let a = values[ip+1] as usize;
        let b = values[ip+2] as usize;
        let destination = values[ip+3] as usize;
        match opcode {
            1 => {
                values[destination] = values[a] + values[b];
                ip += 4;
            },
            2 => {
                values[destination] = values[a] * values[b];
                ip += 4;
            },
            99 => {
                println!("End, pos[0] = {}", values[0]);
                break;
            },
            _ => {
                println!("An error has occurred!");
                break;
            },
        }
    }
    return values;
}

fn main() {
    'outer: for a in 0..100 {
        for b in 0..100 {
            let mut values = read_instructions(String::from("opcodes.txt"));
            values[1] = a;
            values[2] = b;
            values = run(values);
            if values[0] == 19690720 {
                println!("({}, {}) is the answer!", a, b);
                break 'outer;
            }
        }
    }
}
