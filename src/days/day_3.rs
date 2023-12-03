use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};

#[cfg(test)]
#[path = "../unit_tests/day_3.rs"]
mod day_3_tests;

fn split_string_to_numbers(input: &str) -> Vec<(usize, usize, i32)> {
    let mut start_idx = None;
    let mut end_idx = None;

    input
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| {
            if c.is_numeric() {
                if start_idx.is_none() {
                    start_idx = Some(idx);
                }
                end_idx = Some(idx);
                if idx == input.len() - 1 {
                    return Some((
                        start_idx.unwrap(),
                        end_idx.unwrap(),
                        input[start_idx.unwrap()..=end_idx.unwrap()]
                            .parse::<i32>()
                            .unwrap(),
                    ));
                }
                None
            } else if let (Some(start), Some(end)) = (start_idx.take(), end_idx.take()) {
                Some((start, end, input[start..=end].parse::<i32>().unwrap()))
            } else {
                None
            }
        })
        .collect()
}

fn is_next_to_symbol(s: &char) -> bool {
    !s.is_numeric() && *s != '.'
}

pub fn check_surroundings(
    part_numbers: &Vec<Vec<(usize, usize, i32)>>,
    lines: &Vec<String>,
) -> (Vec<i32>, HashMap<u64, Vec<i32>>) {
    let mut result_parts: Vec<i32> = Vec::new();
    let mut gears: HashMap<u64, Vec<i32>> = HashMap::new();

    for (id, line) in lines.iter().enumerate() {
        let part_number_by_row = &part_numbers[id];
        let current_line: Vec<char> = line.chars().collect();

        'this_loop: for part_number in part_number_by_row.iter() {
            let (start, end, value) = part_number;
            let prev_line: Option<Vec<char>> = if id > 0 {
                Some(lines[id - 1].chars().collect())
            } else {
                None
            };
            let next_line: Option<Vec<char>> = if id < lines.len() - 1 {
                Some(lines[id + 1].chars().collect())
            } else {
                None
            };

            // check left
            if *start > 0 && is_next_to_symbol(&current_line[*start - 1]) {
                update_map(&mut gears, id, *start - 1, *value);
                result_parts.push(*value);
                continue 'this_loop;
            }

            // check right
            if *end < line.len() - 1 && is_next_to_symbol(&current_line[*end + 1]) {
                update_map(&mut gears, id, *end + 1, *value);
                result_parts.push(*value);
                continue 'this_loop;
            }

            let start = if *start > 0 { *start - 1 } else { *start };
            let end = if *end < line.len() - 1 {
                *end + 1
            } else {
                *end
            };

            for i in start..=end {
                // check top
                if let Some(l) = &prev_line {
                    if is_next_to_symbol(&l[i]) {
                        update_map(&mut gears, id - 1, i, *value);
                        result_parts.push(*value);
                        continue 'this_loop;
                    }
                }

                // check bottom
                if let Some(l) = &next_line {
                    if is_next_to_symbol(&l[i]) {
                        update_map(&mut gears, id + 1, i, *value);
                        result_parts.push(*value);
                        continue 'this_loop;
                    }
                }
            }
        }
    }
    
    (result_parts, gears)
}

pub fn calculate_sum(numbers: &Vec<i32>) -> i32 {
    numbers.iter().sum()
}

pub fn calculate_product(numbers: &Vec<i32>) -> i32 {
    numbers.iter().product()
}

pub fn calculate_sum_of_gear_ratios(gears: &HashMap<u64, Vec<i32>>) -> i32 {
    let mut sum = 0;
    for gear in gears.values() {
        if gear.len() == 2 {
            sum += calculate_product(gear);
        }
    }
    
    sum
}

fn update_map(map: &mut HashMap<u64, Vec<i32>>, row: usize, col: usize, value: i32) {
    let hash_value = hash_matrix_index(row, col);

    map.entry(hash_value)
        .and_modify(|v| v.push(value))
        .or_insert(vec![value]);
}

pub fn hash_matrix_index(row: usize, col: usize) -> u64 {
    let mut hasher = DefaultHasher::new();
    row.hash(&mut hasher);
    col.hash(&mut hasher);
    hasher.finish()
}

pub fn process_file(file_path: &str) -> io::Result<(Vec<Vec<(usize, usize, i32)>>, Vec<String>)> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut part_numbers = Vec::new();
    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers = split_string_to_numbers(&line);
        part_numbers.push(numbers);
        lines.push(line);
    }

    Ok((part_numbers, lines))
}
