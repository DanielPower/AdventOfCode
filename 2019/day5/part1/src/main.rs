use std::fs;
use std::io;
use std::string::String;

fn read_instructions(filename: String) -> Vec<u32> {
    let mut values = Vec::new();
    {
        let input = fs::read_to_string(String::from(filename))
            .expect("Something went wrong reading the file");
        let mut string_value = String::new();
        for character in input.chars() {
            if character == ',' {
                values.push(string_value.parse::<u32>().unwrap());
                string_value = String::new();
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
    println!("{}", ip);
    loop {
        match values[ip] {
            1 => {
                let a = values[ip + 1] as usize;
                let b = values[ip + 2] as usize;
                let destination = values[ip + 3] as usize;
                values[destination] = values[a] + values[b];
                println!("[ADD {} + {} -> {}]", a, b, destination);
                ip += 4;
            }
            2 => {
                let a = values[ip + 1] as usize;
                let b = values[ip + 2] as usize;
                let destination = values[ip + 3] as usize;
                values[destination] = values[a] * values[b];
                println!("[MUL {} * {} -> {}]", a, b, destination);
                ip += 4;
            }
            3 => {
                println!("Input a value: ");
                let mut input_string = String::new();
                io::stdin()
                    .read_line(&mut input_string)
                    .expect("Failed to read line");
                println!("{}", input_string);
                let input = input_string.trim().parse::<u32>().unwrap();
                let destination = values[ip + 1] as usize;
                println!("[STO {} -> {}]", input, destination);
                values[destination] = input;
                ip += 2;
            }
            4 => {
                let target = values[ip + 1] as usize;
                println!("OUT {}", values[target]);
                ip += 2;
            }
            99 => {
                break;
            }
            _ => {
                println!("Unknown opcode {}", values[ip]);
                break;
            }
        }
    }
    return values;
}

fn main() {
    let values = read_instructions(String::from("opcodes.txt"));
    println!("Running");
    run(values);
    println!("Done");
}
