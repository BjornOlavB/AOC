use std::fs::File;
use std::i32;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _day1 = day1();

    //Should`ve realized using a Hashmap would`ve been easier. Ohh well.. Lesson learned

    let path = Path::new("input");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut positions: HashMap<(i32, i32), i32> = HashMap::new();
    let mut santa_pos = (0, 0);
    let mut robo_pos = (0, 0);

    positions.insert(santa_pos, 2);

    if let Some(line) = reader.lines().next() {
        let line = line?;
        for (i, movement) in line.chars().enumerate() {
            let current_pos = if i % 2 == 0 { &mut santa_pos } else { &mut robo_pos };

            match movement {
                '>' => current_pos.0 += 1,
                '<' => current_pos.0 -= 1,
                '^' => current_pos.1 += 1,
                'v' => current_pos.1 -= 1,
                _ => (),
            }

            *positions.entry(*current_pos).or_insert(0) += 1;
        }
    }

    println!("Houses with at least one present: {}", positions.len());
    Ok(())
}

fn day1() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("input");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    
    let mut row_index = 2;
    let mut col_index = 2;

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; 3]; 3];

    if let Some(line) = reader.lines().next() {
        let line = line?;

        ensure_matrix_size(&mut matrix, row_index, col_index);
        matrix[row_index][col_index] += 1;
        for movement in line.chars() {
            match movement{               
                '>' => col_index += 1,
                '^' => row_index -= 1,
                '<' => col_index -= 1,
                'v' => row_index += 1,
            _ => (),
            }

           if row_index <= 0 || col_index <= 0 {
                double_and_recenter(&mut matrix, &mut row_index, &mut col_index)
           }

            ensure_matrix_size(&mut matrix, row_index, col_index);

            if row_index < matrix.len() && col_index < matrix[row_index].len() {
                matrix[row_index][col_index] += 1;
            }
        }
        check_matrix(&mut matrix)
    }
    Ok(())
}

fn ensure_matrix_size(matrix: &mut Vec<Vec<i32>>, row: usize, col: usize) {
    while matrix.len() <= row {
        matrix.push(vec![0; matrix.first().map_or(1, Vec::len)]);
    }
    for r in matrix.iter_mut() {
        while r.len() <= col {
            r.push(0);
        }
    }
}

fn check_matrix(matrix: &[Vec<i32>]) {
    let mut count = 0;
    for row in matrix {
        for &house in row {
            if house != 0 {
                count += 1;
            }
        }
    }
    println!("Non-zero count: {}", count);
}

fn double_and_recenter(matrix: &mut Vec<Vec<i32>>, row_index: &mut usize, col_index: &mut usize) {
    let old_rows = matrix.len();
    let old_cols = matrix[0].len();
    let new_rows = old_rows * 2;
    let new_cols = old_cols * 2;

    let mut new_matrix = vec![vec![0; new_cols]; new_rows];
    let row_offset = old_rows / 2;
    let col_offset = old_cols / 2;

    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            new_matrix[i + row_offset][j + col_offset] = val;
        }
    }
    *row_index += row_offset;
    *col_index += col_offset;
    *matrix = new_matrix;
}

/*
--- Day 3: Perfectly Spherical Houses in a Vacuum ---

Santa is delivering presents to an infinite two-dimensional grid of houses.

He begins by delivering a present to the house at his starting location, and then an elf at the North Pole calls him via radio and tells him where to move next.
Moves are always exactly one house to the north (^), south (v), east (>), or west (<). After each move, he delivers another present to the house at his new location.

However, the elf back at the north pole has had a little too much eggnog, and so his directions are a little off,
and Santa ends up visiting some houses more than once. How many houses receive at least one present?

For example:

    > delivers presents to 2 houses: one at the starting location, and one to the east.
    ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
    ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.
*/

