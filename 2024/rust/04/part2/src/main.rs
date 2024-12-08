use std::io::Read;

struct Grid {
    items: Vec<char>,
    width: usize,
    height: usize,
}
impl Grid {
    fn from_reader<R: Read>(reader: R) -> Self {
        let mut items: Vec<char> = Vec::new();
        let mut height = 0;
        let mut last_char = 'x';
        for byte in reader.bytes() {
            match byte {
                Ok(b) => {
                    let char = b as char;
                    if char == '\n' {
                        if last_char == '\n' {
                            break;
                        }
                        height += 1;
                    } else {
                        items.push(b as char);
                    }
                    last_char = char;
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        let width = items.len() / height;
        Self {
            items,
            width,
            height,
        }
    }
    fn get(&self, x: usize, y: usize) -> char {
        self.items[y * self.width + x]
    }
}

fn main() {
    let grid = Grid::from_reader(std::io::stdin());
    let mut total = 0;
    for x in 0..(grid.width - 2) {
        for y in 0..(grid.height - 2) {
            let string_1 = format!(
                "{}{}{}",
                grid.get(x, y),
                grid.get(x + 1, y + 1),
                grid.get(x + 2, y + 2)
            );
            let string_2 = format!(
                "{}{}{}",
                grid.get(x + 2, y),
                grid.get(x + 1, y + 1),
                grid.get(x, y + 2),
            );
            if (string_1 == "MAS" || string_1 == "SAM") && (string_2 == "MAS" || string_2 == "SAM")
            {
                total += 1;
            }
        }
    }
    println!("{}", total);
}
