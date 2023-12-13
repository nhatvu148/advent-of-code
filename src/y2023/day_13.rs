use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process_file(filename: &str) -> Vec<(Vec<Vec<char>>, Option<usize>, Option<usize>)> {
    let file = File::open(filename).expect("Unable to open the file.");
    let reader = BufReader::new(file);

    let mut result = Vec::new();
    let mut current_block = Vec::new();

    for line in reader.lines() {
        if let Ok(row) = line {
            if row.is_empty() {
                // Empty line indicates the end of a block, so add the current_block to the result
                process_block(&mut result, &mut current_block);
            } else {
                // Non-empty line, add each char to the current_block
                current_block.push(row.chars().collect());
            }
        }
    }

    // Add the last block if it's not empty
    process_block(&mut result, &mut current_block);

    result
}

fn process_block(
    result: &mut Vec<(Vec<Vec<char>>, Option<usize>, Option<usize>)>,
    current_block: &mut Vec<Vec<char>>,
) {
    if !current_block.is_empty() {
        let transposed = transpose_matrix(&current_block);
        let horizontal_reflection = find_vertical_reflection(&transposed);
        let vertical_reflection = find_vertical_reflection(&current_block);

        result.push((
            current_block.clone(),
            horizontal_reflection,
            vertical_reflection,
        ));
        current_block.clear();
    }
}


pub fn transpose_matrix(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    // Create a new matrix with swapped rows and columns
    let mut transposed_matrix = vec![vec!['0'; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed_matrix[j][i] = matrix[i][j];
        }
    }

    transposed_matrix
}

pub fn find_vertical_reflection(block: &Vec<Vec<char>>) -> Option<usize> {
    let columns = block[0].len();
    if columns < 2 {
        return None;
    }

    for i in 0..columns {
        let mut left = i as i32;
        let mut right = left + 1;

        'this_loop: while left >= 0 && right <= columns as i32 - 1 {
            for row in block {
                let mut temp_left = left;
                let mut temp_right = right;

                while temp_left >= 0 && temp_right <= columns as i32 - 1 {
                    // println!(
                    //     "i {}, left {}, right {}, row {:?}, {} vs {}",
                    //     i,
                    //     temp_left,
                    //     temp_right,
                    //     row,
                    //     row[temp_left as usize],
                    //     row[temp_right as usize]
                    // );
                    if row[temp_left as usize] == row[temp_right as usize] {
                        temp_left -= 1;
                        temp_right += 1;
                    } else {
                        break 'this_loop;
                    }
                }
            }

            // Increment/decrement indices for the next comparison
            left -= 1;
            right += 1;

            // If the loop completes without breaking, return i
            if left < 0 || right > columns as i32 - 1 {
                return Some(i);
            }
        }
    }

    None // Return None if no reflection line is found
}

pub fn smudge_block(block: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<Vec<char>> {
    let mut new_block = block.clone();

    // Change the element at the specified position
    new_block[i][j] = if new_block[i][j] == '#' {
        '.'
    } else {
        '#'
    };

    new_block
}

pub fn get_sum(
    result: &Vec<(Vec<Vec<char>>, Option<usize>, Option<usize>)>,
) -> usize {
    let mut sum = 0;

    // Print the result in 2D format along with reflection lines
    for (block, horizontal_reflection, vertical_reflection) in result {
        println!("Block:");
        for row in block {
            for ch in row {
                print!("{} ", ch);
            }
            println!();
        }

        if let Some(h_line) = horizontal_reflection {
            println!("Horizontal Reflection Line: Row {}", h_line);
            sum += 100 * (h_line + 1);
        } else {
            println!("No Horizontal Reflection Line");
        }

        if let Some(v_line) = vertical_reflection {
            sum += v_line + 1;
            println!("Vertical Reflection Line: Column {}", v_line);
        } else {
            println!("No Vertical Reflection Line");
        }

        println!();
    }

    sum
}
