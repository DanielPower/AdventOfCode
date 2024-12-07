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
    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x < self.width && y < self.height {
            Some(self.items[y * self.width + x])
        } else {
            None
        }
    }
    fn rows(&self) -> impl Iterator<Item = Vec<char>> + '_ {
        (0..self.height).map(move |i| self.items[i * self.width..(i + 1) * self.width].to_vec())
    }
    fn cols(&self) -> impl Iterator<Item = Vec<char>> + '_ {
        (0..self.width).map(move |i| {
            self.items
                .iter()
                .skip(i)
                .step_by(self.width)
                .cloned()
                .collect::<Vec<char>>()
        })
    }
    fn diags(&self) -> impl Iterator<Item = Vec<char>> + '_ {
        (0..self.width + self.height - 1)
            .map(move |i| {
                let mut diag = Vec::new();
                for y in 0..self.height {
                    let x = i as isize - y as isize;
                    if x >= 0 && x < self.width as isize {
                        diag.push(self.get(x as usize, y).unwrap());
                    }
                }
                diag
            })
            .chain((1..self.width + self.height - 1).map(move |i| {
                let mut diag = Vec::new();
                for y in 0..self.height {
                    let x = i as isize - self.height as isize + 1 + y as isize;
                    if x >= 0 && x < self.width as isize {
                        diag.push(self.get(x as usize, y).unwrap());
                    }
                }
                diag
            }))
    }
}

fn main() {
    let grid = Grid::from_reader(std::io::stdin());
    let mut total = 0;
    for chars in grid.rows().chain(grid.cols()).chain(grid.diags()) {
        let string = String::from_iter(chars);
        total += string.matches("XMAS").count() + string.matches("SAMX").count();
    }
    println!("{}", total);
}
