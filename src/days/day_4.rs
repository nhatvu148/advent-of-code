use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn process_file(path: &str) -> Result<(Vec<Vec<u32>>, Vec<Vec<u32>>), io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut winning_numbers = Vec::new();
    let mut having_numbers = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(':').map(|s| s.trim());

        if let Some(second_part) = parts.nth(1) {
            let mut sub_parts = second_part.split('|').map(|s| s.trim());

            if let (Some(first_part), Some(second_part)) = (sub_parts.next(), sub_parts.next()) {
                let winning_number: Vec<u32> = first_part
                    .split_whitespace()
                    .map(|num_str| num_str.parse().unwrap_or(0))
                    .collect();

                let having_number: Vec<u32> = second_part
                    .split_whitespace()
                    .map(|num_str| num_str.parse().unwrap_or(0))
                    .collect();

                winning_numbers.push(winning_number);
                having_numbers.push(having_number);
            }
        }
    }

    Ok((winning_numbers, having_numbers))
}

pub fn find_matching_card(
    winning_numbers: &Vec<Vec<u32>>,
    having_numbers: &Vec<Vec<u32>>,
) -> (i32, u32) {
    let mut points = 0;
    let mut scratch_cards_map: BTreeMap<usize, u32> = winning_numbers
        .iter()
        .enumerate()
        .map(|(id, _)| (id, 1))
        .collect();

    for (row_id, having_number_row) in having_numbers.iter().enumerate() {
        let winning_number_row = &winning_numbers[row_id];
        let mut count: usize = 0;
        for hn in having_number_row.iter() {
            if winning_number_row.contains(&hn) {
                count += 1;
            }
        }
        if count > 0 {
            let from_index = row_id + 1;
            let to_index = if row_id + count > having_numbers.len() - 1 {
                having_numbers.len() - 1
            } else {
                row_id + count
            };
            for key in scratch_cards_map.clone().keys() {
                if *key >= from_index && *key <= to_index {
                    let copy_scratch_cards_map = scratch_cards_map.clone();
                    let current_row_value = copy_scratch_cards_map.get(&row_id).unwrap();
                    update_map(&mut scratch_cards_map, *key, current_row_value * 1);
                }
            }

            let base: i32 = 2;
            let exponent = count - 1;
            points += base.pow(exponent as u32);
        }
    }

    let total_scratch_cards: u32 = scratch_cards_map.values().sum();

    (points, total_scratch_cards)
}

fn update_map(map: &mut BTreeMap<usize, u32>, key: usize, value: u32) {
    map.entry(key).and_modify(|v| *v += value);
}
