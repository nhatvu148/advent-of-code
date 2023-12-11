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

    galaxies
}

fn get_indices_without_galaxies(
    galaxies: &Vec<Vec<char>>,
) -> (Vec<usize>, Vec<usize>) {
    let rows_with_galaxies: Vec<bool> =
        galaxies.iter().map(|row| row.contains(&'#')).collect();

    // Get indices of rows without galaxies
    let rows_without_galaxies_indices: Vec<usize> = rows_with_galaxies
        .iter()
        .enumerate()
        .filter_map(
            |(index, &has_galaxies)| {
                if !has_galaxies {
                    Some(index)
                } else {
                    None
                }
            },
        )
        .collect();

    // Get indices of columns without galaxies
    let mut cols_without_galaxies_indices = Vec::new();
    for col in 0..galaxies[0].len() {
        let has_galaxies_in_col = galaxies.iter().any(|row| row[col] == '#');
        if !has_galaxies_in_col {
            cols_without_galaxies_indices.push(col);
        }
    }

    (rows_without_galaxies_indices, cols_without_galaxies_indices)
}

fn expand_galaxy(galaxies: Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    let rows_with_galaxies: Vec<bool> =
        galaxies.iter().map(|row| row.contains(&'#')).collect();

    // Expand rows
    let mut new_galaxies: Vec<Vec<char>> = Vec::new();
    for (row, has_galaxies) in galaxies.iter().zip(rows_with_galaxies.iter()) {
        if *has_galaxies {
            new_galaxies.push(row.clone());
        } else {
            for _ in 0..n {
                new_galaxies.push(row.clone());
            }
        }
    }

    // Expand columns
    let num_cols = new_galaxies[0].len();
    for col in (0..num_cols).rev() {
        let has_galaxies_in_col =
            new_galaxies.iter().any(|row| row[col] == '#');
        if !has_galaxies_in_col {
            for row in &mut new_galaxies {
                let character = row[col];
                for i in 1..n {
                    row.insert(col + i, character);
                }
            }
        }
    }

    new_galaxies
}

fn find_galaxies(galaxies: &Vec<Vec<char>>) -> Vec<Galaxy> {
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

fn find_shortest_path(g1: Galaxy, g2: Galaxy) -> (Vec<Galaxy>, usize) {
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

pub fn get_sum_length(galaxies_map: &Vec<Vec<char>>) -> usize {
    let galaxies_to_expand = galaxies_map.clone();
    let expanded_galaxies_map = expand_galaxy(galaxies_to_expand, 2);
    // Print the galaxies for demonstration
    // for row in &expanded_galaxies_map {
    //     for cell in row {
    //         print!("{}", cell);
    //     }
    //     println!();
    // }

    let galaxies = find_galaxies(&expanded_galaxies_map);
    // println!("{:?}", galaxies);

    let mut sum_length = 0;
    // Find and print the shortest paths between all pairs of galaxies
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in &galaxies[i + 1..] {
            let (_path, length) = find_shortest_path(*g1, *g2);
            // println!(
            //     "Shortest path between {:?} and {:?}: {:?}, Length: {}",
            //     g1, g2, path, length
            // );
            sum_length += length;
        }
    }

    sum_length
}

pub fn get_sum_length_n(galaxies_map: &Vec<Vec<char>>, n: usize) -> usize {
    let (row_indices, col_indices) =
        get_indices_without_galaxies(&galaxies_map);
    let galaxies = find_galaxies(&galaxies_map);
    let mut sum_length = 0;

    // Find and print the shortest paths between all pairs of galaxies
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in &galaxies[i + 1..] {
            let (_path, length) =
                find_shortest_path_n(*g1, *g2, &row_indices, &col_indices, n);

            sum_length += length;
        }
    }

    sum_length
}

fn calculate_manhattan_distance_n(
    g1: Galaxy,
    g2: Galaxy,
    rows_without_galaxies: &[usize],
    cols_without_galaxies: &[usize],
    n: usize,
) -> usize {
    let mut distance = (g1.x as isize - g2.x as isize).abs() as usize
        + (g1.y as isize - g2.y as isize).abs() as usize;

    // Check for rows without galaxies between g1 and g2
    if g1.y != g2.y {
        for &_ in rows_without_galaxies
            .iter()
            .filter(|&&r| r > g1.y && r < g2.y)
        {
            distance += n - 1;
        }
        for &_ in rows_without_galaxies
            .iter()
            .filter(|&&r| r > g2.y && r < g1.y)
        {
            distance += n - 1;
        }
    }

    // Check for cols without galaxies between g1 and g2
    if g1.x != g2.x {
        for &_ in cols_without_galaxies
            .iter()
            .filter(|&&c| c > g1.x && c < g2.x)
        {
            distance += n - 1;
        }
        for &_ in cols_without_galaxies
            .iter()
            .filter(|&&c| c > g2.x && c < g1.x)
        {
            distance += n - 1;
        }
    }

    distance
}

fn find_shortest_path_n(
    g1: Galaxy,
    g2: Galaxy,
    rows_without_galaxies: &[usize],
    cols_without_galaxies: &[usize],
    n: usize,
) -> (Vec<Galaxy>, usize) {
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

    let length = calculate_manhattan_distance_n(
        g1,
        g2,
        rows_without_galaxies,
        cols_without_galaxies,
        n,
    );

    (path, length)
}
