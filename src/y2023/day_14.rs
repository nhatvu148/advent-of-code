use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process_file(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("Unable to open the file.");
    let reader = BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        if let Ok(row) = line {
            let mut current_row = Vec::new();

            for ch in row.chars() {
                current_row.push(ch);
            }

            result.push(current_row);
        }
    }

    result
}

pub fn tilt_north(mirror: &Vec<Vec<char>>) -> usize {
    let mut result: Vec<Vec<char>> = Vec::new();
    let col_len = mirror[0].len();

    for mirror in mirror.iter() {
        result.push(mirror.clone());

        for col in 0..col_len {
            let end = result.len() - 1;
            let mut i = result.len() - 1;
            if i == 0 {
                break;
            }

            if result[end][col] == '#' || result[end][col] == '.' {
                continue;
            }

            while i > 0 {
                i -= 1;
                if result[i][col] == '#' || result[i][col] == 'O' {
                    if i + 1 == end {
                        break;
                    }
                    // swap
                    let temp = result[end][col];
                    result[end][col] = result[i + 1][col];
                    result[i + 1][col] = temp;
                    break;
                } else if i == 0 {
                    // swap
                    let temp = result[end][col];
                    result[end][col] = result[i][col];
                    result[i][col] = temp;
                    break;
                }

            }
        }
    }

    println!("tilted:");
    let mut total_load = 0;

    for (i, row) in result.iter().enumerate() {
        println!("{:?}", row);
        total_load += (result.len() - i) * count_o(row);
        println!("row i: {}, total_load {:?}", i, total_load);
    }
    println!("final total_load {:?}", total_load);
    total_load
}

fn count_o(chars: &Vec<char>) -> usize {
    chars.iter().filter(|&&c| c == 'O').count()
}