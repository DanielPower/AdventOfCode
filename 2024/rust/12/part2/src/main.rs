fn parse_input() -> Vec<Vec<char>> {
    let mut plants: Vec<Vec<char>> = Vec::new();
    for (index, line) in std::io::stdin().lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        plants.push(Vec::new());
        for c in line.chars() {
            plants[index].push(c);
        }
    }
    plants
}

fn get_valid_neighbors(x: i32, y: i32, gardens: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut neighbors: Vec<(i32, i32)> = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < (gardens[0].len() - 1) as i32 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < (gardens.len() - 1) as i32 {
        neighbors.push((x, y + 1));
    }
    let value = gardens[y as usize][x as usize];
    neighbors = neighbors
        .into_iter()
        .filter(|(nx, ny)| gardens[*ny as usize][*nx as usize] == value)
        .collect();
    neighbors
}

fn flood_fill(
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    garden: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> (usize, usize) {
    visited[y as usize][x as usize] = true;
    let neighbors = get_valid_neighbors(x, y, garden);
    let mut area = 1;
    // The perimeter is equal to the number of corners. This set of cases can determine the number
    // of corners based on the number and placement of neighbors.
    let mut perimeter = match neighbors.len() {
        // A lonely plant is surounded by 4 corners
        // o-o
        // |#|
        // o-o
        0 => 4,

        // A plant with one neighbor has 2 corners
        // o-
        // |##
        // o-
        1 => 2,

        2 => {
            if neighbors[0].0 == neighbors[1].0 || neighbors[0].1 == neighbors[1].1 {
                // A plant with 2 neighbors has no corners if they are in a line
                // ---
                // ###
                // ---
                0
            } else {
                let dx = neighbors[0].0 as i32 + neighbors[1].0 as i32 - (x as i32) * 2;
                let dy = neighbors[0].1 as i32 + neighbors[1].1 as i32 - (y as i32) * 2;
                // A plant with 2 neighbors has 1 or 2 corners if they are not in a line
                // If the inner diagonal (marked with ?) is in the same region, there is 1 corner.
                // Otherwise there are 2 corners.
                // o--
                // |##
                // |#?
                if garden[y as usize][x as usize] == garden[(y + dy) as usize][(x + dx) as usize] {
                    1
                } else {
                    2
                }
            }
        }
        3 => {
            // A plant with 3 neighbors has 0 to 2 corners, depending on the diagonals in the
            // direction of the T intersection (marked with ?).
            // |#?
            // |##
            // |#?
            let ax = neighbors[0].0 + neighbors[1].0 + neighbors[2].0 - x * 3;
            let ay = neighbors[0].1 + neighbors[1].1 + neighbors[2].1 - y * 3;

            // By taking the average of the three neighbors, we can determine the direction of the
            // T intersection. We can use this to find the two relevant diagonals.
            let (n1x, n1y, n2x, n2y) = if ax == 0 {
                (x - 1, y + ay, x + 1, y + ay)
            } else {
                (x + ax, y - 1, x + ax, y + 1)
            };
            [(n1x, n1y), (n2x, n2y)]
                .iter()
                .filter(|(nx, ny)| {
                    garden[*ny as usize][*nx as usize] != garden[y as usize][x as usize]
                })
                .count()
        }

        // A plant with 4 neighbors has one corner for each diagonal (marked with ?) neighbor that is
        // in a different region.
        // ?#?
        // ###
        // ?#?
        4 => [
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x + 1, y - 1),
            (x + 1, y + 1),
        ]
        .iter()
        .filter(|(nx, ny)| garden[*ny as usize][*nx as usize] != garden[y as usize][x as usize])
        .count(),
        _ => panic!("Invalid number of neighbors"),
    };
    for (nx, ny) in get_valid_neighbors(x, y, garden) {
        if visited[ny as usize][nx as usize] {
            continue;
        }
        let (n_area, n_perimeter) = flood_fill(nx, ny, width, height, garden, visited);
        area += n_area;
        perimeter += n_perimeter;
    }
    return (area, perimeter);
}

fn main() {
    let garden = parse_input();
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];
    let (width, height) = (garden[0].len(), garden.len());
    let mut price = 0;
    for y in 0..height {
        for x in 0..width {
            if !visited[y][x] {
                let (area, perimeter) =
                    flood_fill(x as i32, y as i32, width, height, &garden, &mut visited);
                price += area * perimeter;
            }
        }
    }
    println!("{:?}", price);
}
