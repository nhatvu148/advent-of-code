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

pub fn rotate_mirror_clockwise(mirror: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = mirror.len();
    let cols = mirror[0].len();

    let mut rotated_mirror = vec![vec![' '; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            rotated_mirror[j][rows - 1 - i] = mirror[i][j];
        }
    }

    rotated_mirror
}

pub fn find_cycle_length(mirror: &Vec<Vec<char>>) -> (usize, usize) {
    let mut tortoise = mirror.clone();
    let mut hare = mirror.clone();

    let mut tortoise_load_seq = Vec::new();
    let mut hare_load_seq = Vec::new();

    let mut cycle_length = 0;
    let mut cycle_start = 0;

    loop {
        // Move the tortoise one step
        tortoise = rotate_one_cycle(&tortoise);
        tortoise_load_seq.push(measure_load(&tortoise));

        // Move the hare two steps
        hare = rotate_one_cycle(&hare);
        hare = rotate_one_cycle(&hare);
        hare_load_seq.push(measure_load(&hare));

        if hare_load_seq.len() % 2 == 0 {
            // Check for cycle after every two steps of hare
            if hare_load_seq[cycle_start]
                == hare_load_seq[hare_load_seq.len() - 1]
            {
                // Cycle detected, find the length and start index
                let mut i = cycle_start;
                while hare_load_seq[i] != hare_load_seq[hare_load_seq.len() - 1]
                {
                    i += 1;
                }
                cycle_length = hare_load_seq.len() - cycle_start;
                cycle_start = i;
                break;
            }
        }

        if tortoise_load_seq == hare_load_seq {
            // Cycle detected, find the length and start index
            let mut i = cycle_start;
            while hare_load_seq[i] != hare_load_seq[hare_load_seq.len() - 1] {
                i += 1;
            }
            cycle_length = hare_load_seq.len() - cycle_start;
            cycle_start = i;
            break;
        }
    }

    (cycle_length, cycle_start)
}

pub fn rotate_one_cycle(mirror: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_mirror: Vec<Vec<char>> = mirror.clone();
    // one cycle
    // North -> West -> South -> East
    for _ in 1..=4 {
        new_mirror = tilt_north(&new_mirror);
        new_mirror = rotate_mirror_clockwise(&new_mirror);
    }

    new_mirror
}

pub fn tilt_north(mirror: &Vec<Vec<char>>) -> Vec<Vec<char>> {
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

    result
}

pub fn measure_load(mirror: &Vec<Vec<char>>) -> usize {
    let mut total_load = 0;

    for (i, row) in mirror.iter().enumerate() {
        total_load += (mirror.len() - i) * count_o(row);
    }
    total_load
}

fn count_o(chars: &Vec<char>) -> usize {
    chars.iter().filter(|&&c| c == 'O').count()
}
