enum Stage {
    FileLength,
    EmptySpace,
}

fn parse_input() -> Vec<i64> {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let numbers: Vec<i64> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();
    numbers
}

fn build_disk(numbers: Vec<i64>) -> Vec<i64> {
    let mut stage = Stage::FileLength;
    let mut file_id = 0;
    let mut disk: Vec<i64> = Vec::new();
    for number in numbers {
        match stage {
            Stage::FileLength => {
                for _ in 0..number {
                    disk.push(file_id);
                }
                stage = Stage::EmptySpace;
                file_id += 1;
            }
            Stage::EmptySpace => {
                for _ in 0..number {
                    disk.push(-1);
                }
                stage = Stage::FileLength;
            }
        }
    }
    disk
}

fn defrag_disk(disk: &mut Vec<i64>) {
    let mut index = 0;
    loop {
        match disk.get(index) {
            Some(-1) => loop {
                match disk.pop() {
                    Some(-1) => {}
                    Some(file_id) => {
                        disk[index] = file_id;
                        break;
                    }
                    None => break,
                }
            },
            Some(_) => {}
            None => break,
        }
        index += 1;
    }
}

fn checksum(disk: &Vec<i64>) -> i64 {
    disk.iter()
        .enumerate()
        .fold(0, |acc, (index, file_id)| acc + index as i64 * file_id)
}

fn main() {
    let numbers = parse_input();
    let mut disk = build_disk(numbers);
    defrag_disk(&mut disk);
    println!("{}", checksum(&disk));
}
