use std::fs::File;
use std::io::{self, BufRead};

pub fn process_file(
    file_path: &str,
) -> Result<(Vec<u128>, Vec<u128>), io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut time_vector = Vec::new();
    let mut distance_vector = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() == 2 {
            let values: Vec<u128> = parts[1]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            match parts[0].trim() {
                "Time" => time_vector.extend(values),
                "Distance" => distance_vector.extend(values),
                _ => eprintln!("Unknown category: {}", parts[0]),
            }
        }
    }

    Ok((time_vector, distance_vector))
}

pub fn count_number_of_ways_to_beat_record(
    time_vector: &Vec<u128>,
    distance_vector: &Vec<u128>,
) -> (u128, u128) {
    let mut counts = Vec::new();

    for (id, time) in time_vector.iter().enumerate() {
        let mut mid = 0;
        let mut count = 0;
        if time % 2 != 0 {
            mid = time / 2;
            while mid > 0 {
                if mid * (time - mid) > distance_vector[id] {
                    count += 2;
                } else {
                    break;
                }

                mid -= 1;
            }
        } else {
            mid = time / 2;
            if mid * (time - mid) > distance_vector[id] {
                count += 1;
                mid -= 1;
                while mid > 0 {
                    if mid * (time - mid) > distance_vector[id] {
                        count += 2;
                    } else {
                        break;
                    }

                    mid -= 1;
                }
            }
        }

        counts.push(count);
    }

    let combined_time: u128 = time_vector
        .iter()
        .map(|&num| num.to_string())
        .collect::<String>()
        .parse()
        .unwrap_or_default();
    let combined_distance: u128 = distance_vector
        .iter()
        .map(|&num| num.to_string())
        .collect::<String>()
        .parse()
        .unwrap_or_default();
    let mut combined_counts = Vec::new();

    let mut mid = 0;
    let mut count = 0;
    if combined_time % 2 != 0 {
        mid = combined_time / 2;
        while mid > 0 {
            if mid * (combined_time - mid) > combined_distance {
                count += 2;
            } else {
                break;
            }

            mid -= 1;
        }
    } else {
        mid = combined_time / 2;
        if mid * (combined_time - mid) > combined_distance {
            count += 1;
            mid -= 1;
            while mid > 0 {
                if mid * (combined_time - mid) > combined_distance {
                    count += 2;
                } else {
                    break;
                }

                mid -= 1;
            }
        }
    }

    combined_counts.push(count);

    // println!("counts: {:?}", counts);
    // println!("calculate_product: {:?}", calculate_product(&counts));
    // println!("calculate_combined_counts: {:?}", calculate_product(&combined_counts));

    (
        calculate_product(&counts),
        calculate_product(&combined_counts),
    )
}

fn calculate_product(numbers: &Vec<u128>) -> u128 {
    numbers.iter().product()
}
