enum Stage {
    FileLength,
    EmptySpace,
}

#[derive(Clone, Debug)]
enum Block {
    File(i64, i64),
    Empty(i64),
}

fn parse_input() -> Vec<i64> {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let numbers: Vec<i64> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();
    numbers
}

fn build_disk(numbers: Vec<i64>) -> Vec<Block> {
    let mut stage = Stage::FileLength;
    let mut file_id = 0;
    let mut disk: Vec<Block> = Vec::new();
    for number in numbers {
        match stage {
            Stage::FileLength => {
                disk.push(Block::File(file_id, number));
                stage = Stage::EmptySpace;
                file_id += 1;
            }
            Stage::EmptySpace => {
                if number > 0 {
                    disk.push(Block::Empty(number));
                }
                stage = Stage::FileLength;
            }
        }
    }
    disk
}

fn defrag_disk(mut disk: Vec<Block>) -> Vec<Block> {
    let last_file = match disk.last() {
        Some(Block::File(file_id, file_size)) => (*file_id, *file_size),
        _ => panic!("Expected file block"),
    };
    let mut move_file_id = last_file.0;
    loop {
        if move_file_id == 0 {
            break;
        }
        let move_file_index = disk
            .iter()
            .position(|block| match block {
                Block::File(file_id, _) => move_file_id == *file_id,
                _ => false,
            })
            .unwrap();
        move_file_id -= 1;
        let (file_id, file_size) = match disk[move_file_index] {
            Block::Empty(_) => {
                continue;
            }
            Block::File(file_id, file_size) => (file_id, file_size),
        };
        let empty_space_index: Option<usize> = disk.iter().position(|block| match block {
            Block::Empty(empty_size) => {
                if *empty_size >= file_size {
                    true
                } else {
                    false
                }
            }
            _ => false,
        });
        match empty_space_index {
            Some(index) => {
                if index > move_file_index {
                    continue;
                }
                let empty_size = match disk[index] {
                    Block::Empty(empty_size) => empty_size,
                    _ => panic!("Expected empty block"),
                };
                disk[move_file_index] = Block::Empty(file_size);
                disk.remove(index);
                disk.insert(index, Block::File(file_id, file_size));
                if empty_size > file_size {
                    disk.insert(index + 1, Block::Empty(empty_size - file_size));
                }
            }
            None => {}
        }
    }
    disk
}

fn expand_disk(disk: &Vec<Block>) -> Vec<i64> {
    let mut expanded_disk: Vec<i64> = Vec::new();
    for block in disk {
        match block {
            Block::File(file_id, file_size) => {
                for _ in 0..*file_size {
                    expanded_disk.push(*file_id);
                }
            }
            Block::Empty(empty_size) => {
                for _ in 0..*empty_size {
                    expanded_disk.push(0);
                }
            }
        }
    }
    expanded_disk
}

fn checksum(disk: &Vec<i64>) -> i64 {
    disk.iter()
        .enumerate()
        .fold(0, |acc, (index, file_id)| acc + index as i64 * file_id)
}

fn main() {
    let numbers = parse_input();
    let mut disk = build_disk(numbers);
    disk = defrag_disk(disk);
    let expanded_disk = expand_disk(&disk);
    println!("{}", checksum(&expanded_disk));
}
