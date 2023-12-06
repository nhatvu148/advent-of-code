use std::fs::File;
use std::io::{self, BufRead};

pub fn process_file(
    file_path: &str,
) -> Result<(Vec<u32>, Vec<u32>), io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut time_vector = Vec::new();
    let mut distance_vector = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() == 2 {
            let values: Vec<u32> = parts[1]
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
    time_vector: &Vec<u32>,
    distance_vector: &Vec<u32>,
) -> (u32, u32) {
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

    println!("counts: {:?}", counts);
    println!("calculate_product: {:?}", calculate_product(&counts));

    (calculate_product(&counts), 0)
}

fn calculate_product(numbers: &Vec<u32>) -> u32 {
    numbers.iter().product()
}
