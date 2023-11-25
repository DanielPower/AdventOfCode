use std::string::String;

fn main() {
    let mut valid_values: u32 = 0;
    for value in 193651..649729+1 {
        let value_string: String = value.to_string();
        let mut has_duplicate: bool = false;
        let mut never_decreases: bool = true;
        for index in 0..value_string.len() {
            if index != 0 {
                // Check for duplicate neighbor
                if value_string[index-1 as usize] == value_string[index as usize] {
                    has_duplicate = true;
                }

                // Check that never decreases
                if value_string[index-1 as usize] > value_string[index as usize] {
                    never_decreases = false;
                }
            }
        }

        if has_duplicate && never_decreases {
            valid_values += 1;
        }
    }
}
