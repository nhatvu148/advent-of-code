use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Galaxy {
    pub x: usize,
    pub y: usize,
}

pub fn process_file(filename: &str) -> Vec<Vec<char>> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("Unable to open the file."),
    };

    let reader = io::BufReader::new(file);
    let mut galaxies: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        if let Ok(row) = line {
            let galaxy_row: Vec<char> = row.chars().collect();
            galaxies.push(galaxy_row);
        }
    }

    let rows_with_galaxies: Vec<bool> =
        galaxies.iter().map(|row| row.contains(&'#')).collect();

    // Double the size of rows without galaxies
    let mut new_galaxies: Vec<Vec<char>> = Vec::new();
    for (row, has_galaxies) in galaxies.iter().zip(rows_with_galaxies.iter()) {
        if *has_galaxies {
            new_galaxies.push(row.clone());
        } else {
            new_galaxies.push(row.clone());
            new_galaxies.push(row.clone());
        }
    }

    // Double the size of columns without galaxies
    let num_cols = new_galaxies[0].len();
    for col in (0..num_cols).rev() {
        let has_galaxies_in_col =
            new_galaxies.iter().any(|row| row[col] == '#');
        if !has_galaxies_in_col {
            for row in &mut new_galaxies {
                let character = row[col];
                row.insert(col + 1, character); // Insert the same character again to double the size
            }
        }
    }

    new_galaxies
}

pub fn find_galaxies(galaxies: &Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut galaxy_coordinates: Vec<Galaxy> = Vec::new();

    for (y, row) in galaxies.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '#' {
                galaxy_coordinates.push(Galaxy { x, y });
            }
        }
    }

    galaxy_coordinates
}

fn calculate_manhattan_distance(g1: Galaxy, g2: Galaxy) -> usize {
    (g1.x as isize - g2.x as isize).abs() as usize
        + (g1.y as isize - g2.y as isize).abs() as usize
}

pub fn find_shortest_path(g1: Galaxy, g2: Galaxy) -> (Vec<Galaxy>, usize) {
    let mut path = Vec::new();

    let dx = g1.x as isize - g2.x as isize;
    let dy = g1.y as isize - g2.y as isize;

    for step in 0..=min(dx.abs(), dy.abs()) as usize {
        let next_x = if dx < 0 { g1.x + step } else { g1.x - step };
        let next_y = if dy < 0 { g1.y + step } else { g1.y - step };
        path.push(Galaxy {
            x: next_x,
            y: next_y,
        });
    }

    let length = calculate_manhattan_distance(g1, g2);

    (path, length)
}
