use std::{fs::File, io::{BufRead, BufReader}, collections::HashSet};

use nalgebra::DMatrix;

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

const GEAR_CENTER: char = '*';

fn part_2(file: &str) -> i32 {
    let file = File::open(file).expect("Error opening file");
    let lines = BufReader::new(file).lines();
    let mut data = Vec::new();
    let mut nrows = 0;
    for line in lines {
        data.extend(line.unwrap().chars());
        nrows += 1;
    }
    let matrix = DMatrix::from_row_slice(nrows, data.len() / nrows, &data);
    let mut potential_gears = Vec::new();
    for row in 0..matrix.nrows() {
        for col in 0..matrix.ncols() {
            if matrix[(row, col)] == GEAR_CENTER {
                potential_gears.push((row as i32, col as i32));
            }
        }
    }
    let mut sum = 0;
    for (row, col) in potential_gears {
        let mut visited = HashSet::<(i32, i32)>::new();
        let mut parts = Vec::<>::new();
        let neighbors = Vec::from(&[
            (row - 1, col- 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ]);
        for neighbor in neighbors {
            if visited.contains(&neighbor) || neighbor.0 < 0 || neighbor.1 < 0 || neighbor.0 >= matrix.nrows() as i32 || neighbor.1 >= matrix.ncols() as i32 {
                continue;
            }
            let neighbor = (neighbor.0 as usize, neighbor.1 as usize);
            if matrix[(neighbor.0 as usize, neighbor.1 as usize)].is_digit(10) {
                let mut num = matrix[neighbor].clone().to_string();
                // Until we hit a non-digit, keep adding to the number with neighbors to the left and right of the current cell
                let mut x_offset = neighbor.1 as i32 + 1;
                while x_offset < matrix.ncols() as i32 && matrix[(neighbor.0, x_offset as usize)].is_digit(10) {
                    num.push(matrix[(neighbor.0, x_offset as usize)]);
                    visited.insert((neighbor.0 as i32, x_offset as i32));
                    x_offset += 1;
                }
                let mut x_offset = neighbor.1 as i32 - 1;
                while x_offset >= 0 && matrix[(neighbor.0, x_offset as usize)].is_digit(10) {
                    num.insert(0, matrix[(neighbor.0, x_offset as usize)]);
                    visited.insert((neighbor.0 as i32, x_offset as i32));
                    x_offset -= 1;
                }
                parts.push(num.parse::<i32>().unwrap());
            }
        }
        if parts.len() == 2 {
            sum += parts[0] * parts[1];
        }
    }
    sum
}

fn main() {
    println!("{}", part_1("example.txt"));
    println!("{}", part_1("input.txt"));
    println!("{}", part_2("example.txt"));
    println!("{}", part_2("input.txt"));
}   
