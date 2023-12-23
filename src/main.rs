use std::{fs::File, io::BufRead};

fn get_matrix(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("File not found");
    std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

// I'm not gonna bother rewriting this, but this was before I learned that there was a crate that does matrices
fn part_1(filename: &str) -> i32 {
    let mut matrix = get_matrix(filename);
    let mut sum = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let c = matrix[y][x];
            if c.is_digit(10) {
                let mut symbol_found = false;
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let adj_x = x as i32 + i;
                        let adj_y = y as i32 + j;
                        if adj_x < 0 || adj_y < 0 || adj_x >= matrix[y].len() as i32 || adj_y >= matrix.len() as i32 {
                            continue;
                        }
                        let adj_cell = matrix[adj_y as usize][adj_x as usize];
                        if adj_cell.is_digit(10) || adj_cell == '.' {
                            continue;
                        } else {
                            let mut num = String::from(matrix[y][x]);
                            matrix[y][x] = '.';
                            let mut x_offset = x + 1;
                            while x_offset < matrix[y].len() && matrix[y][x_offset].is_digit(10) {
                                num.push(matrix[y][x_offset]);
                                matrix[y][x_offset] = '.';
                                x_offset += 1;
                            }
                            let mut x_offset = (x - 1) as i32;
                            while x_offset >= 0 && matrix[y][x_offset as usize].is_digit(10) {
                                num.insert(0, matrix[y][x_offset as usize]);
                                matrix[y][x_offset as usize] = '.';
                                x_offset -= 1;
                            }
                            sum += num.parse::<i32>().unwrap();
                            symbol_found = true;
                        }
                        if symbol_found {
                            break;
                        }
                    }
                    if symbol_found {
                        break;
                    }
                }
            }
        }
    }
    sum
}



fn main() {
    assert_eq!(part_1("example.txt"), 4361);
    assert_eq!(part_1("input.txt"), 528819);
    // assert_eq!(part2(matrix), 467835);
    // println!("{}", part2(matrix));
}   
