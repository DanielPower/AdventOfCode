use std::fs;
use std::string::String;

struct Point {
    x: i32,
    y: i32,
    dist: i32,
}

fn read_input(filename: String) -> Vec<Vec<Point>> {
    let mut wires = vec![
        vec![Point {x: 0, y: 0, dist: 0}],
        vec![Point {x: 0, y: 0, dist: 0}],
    ];

    let input = fs::read_to_string(String::from(filename))
        .expect("Something went wrong reading the file");

    let mut dir = Point {x: 1, y: 1, dist: 0};
    let mut current_wire: u8 = 0;
    let mut string_value = String::from("");
    for character in input.chars() {
        match character {
            'L' => dir = Point {x: -1, y: 0, dist: 0},
            'R' => dir = Point {x: 1, y: 0, dist: 0},
            'U' => dir = Point {x: 0, y: -1, dist: 0},
            'D' => dir = Point {x: 0, y: 1, dist: 0},
            ',' | '\n' => {
                let value = string_value.parse::<i32>().unwrap();
                string_value = String::from("");
                let prev_point = wires[current_wire as usize].last().unwrap();
                let point = Point {
                    x: prev_point.x + dir.x * value,
                    y: prev_point.y + dir.y * value,
                    dist: prev_point.dist + value,
                };
                wires[current_wire as usize].push(point);
            },
            _ => string_value.push(character),
        }
        if character == '\n' {
            current_wire += 1;
        }
    }

    return wires;
}

fn main() {
    let wires = read_input(String::from("input.txt"));
    let mut intersections: Vec<Point> = Vec::new();
    for i in 0..wires[0].len()-1 {
        let a1 = &wires[0][i];
        let a2 = &wires[0][i+1];

        for j in 0..wires[1].len()-1 {
            let b1 = &wires[1][j];
            let b2 = &wires[1][j+1];

            if a1.x == a2.x && b1.y == b2.y
            && ((a1.y < b1.y && b1.y < a2.y) || (a1.y > b1.y && b1.y > a2.y))
            && ((b1.x < a1.x && a1.x < b2.x) || (b1.x > a1.x && a1.x > b2.x)) {
                intersections.push(Point {x: a1.x, y: b1.y, dist: a1.dist + b1.dist + (a1.x - b1.x).abs() + (a1.y - b1.y).abs()});
            } else if a1.y == a2.y && b1.x == b2.x
            && ((a1.x < b1.x && b1.x < a2.x) || (a1.x > b1.x && b1.x > a2.x))
            && ((b1.y < a1.y && a1.y < b2.y) || (b1.y > a1.y && a1.y > b2.y)) {
                intersections.push(Point {x: b1.x, y: a1.y, dist: a1.dist + b1.dist + (a1.x - b1.x).abs() + (a1.y + b1.y).abs()});
            }
        }
    }

    let mut min = i32::max_value();
    for intersection in intersections {
        if intersection.dist < min {
            min = intersection.dist;
        }
    }

    println!("{}", min);
}
