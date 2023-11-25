use std::fs;
use std::string::String;

fn main() {
    let mut values = Vec::new();
    {
        let input = fs::read_to_string("opcodes.txt")
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
    values[1] = 12;
    values[2] = 2;
    for index in (0..values.len()).step_by(4) {
        let opcode = values[index];
        let a = values[index+1] as usize;
        let b = values[index+2] as usize;
        let destination = values[index+3] as usize;
        println!("({}, {}, {}, {})", values[index], values[index+1], values[index+2], values[index+3]);
        match opcode {
            1 => {
                println!("{} + {} -> {}", a, b, destination);
                values[destination] = values[a] + values[b];
            },
            2 => {
                println!("{} * {} -> {}", values[a], values[b], destination);
                values[destination] = values[a] * values[b];
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
}
