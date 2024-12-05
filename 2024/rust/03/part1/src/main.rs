use std::io::Read;

enum ParserStage {
    M,
    U,
    L,
    BL,
    D1,
    D2,
}

fn main() {
    let mut stage = ParserStage::M;
    let mut total = 0;
    let mut s1 = String::new();
    let mut s2 = String::new();
    while let Some(byte) = std::io::stdin().lock().bytes().next() {
        let char: char = byte.unwrap().into();
        stage = match stage {
            ParserStage::M => {
                if char == 'm' {
                    ParserStage::U
                } else {
                    ParserStage::M
                }
            }
            ParserStage::U => {
                if char == 'u' {
                    ParserStage::L
                } else {
                    ParserStage::M
                }
            }
            ParserStage::L => {
                if char == 'l' {
                    ParserStage::BL
                } else {
                    ParserStage::M
                }
            }
            ParserStage::BL => {
                if char == '(' {
                    ParserStage::D1
                } else {
                    ParserStage::M
                }
            }
            ParserStage::D1 => {
                if char >= '0' && char <= '9' {
                    s1.push(char);
                    ParserStage::D1
                } else if char == ',' {
                    ParserStage::D2
                } else {
                    ParserStage::M
                }
            }
            ParserStage::D2 => {
                if char >= '0' && char <= '9' {
                    s2.push(char);
                    ParserStage::D2
                } else if char == ')' {
                    total += s1.parse::<i32>().unwrap() * s2.parse::<i32>().unwrap();
                    ParserStage::M
                } else {
                    ParserStage::M
                }
            }
        };
        if matches!(stage, ParserStage::M) {
            s1.clear();
            s2.clear();
        }
    }
    println!("{}", total);
}
