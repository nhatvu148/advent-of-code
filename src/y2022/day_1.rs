use std::fs::File;
use std::io::{self, BufRead};

pub fn process_file(file_path: &str) -> (u32, u32) {
    let mut batch: Vec<u32> = Vec::new();
    let mut sums: Vec<u32> = Vec::new();

    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line_content) = line {
                if !line_content.is_empty() {
                    let value: u32 = line_content
                        .split_whitespace()
                        .filter(|s| !s.is_empty())
                        .filter_map(|s| s.parse().ok())
                        .next()
                        .unwrap_or_default();

                    batch.push(value);
                    println!("{:?}", value);
                } else {
                    sums.push(batch.iter().fold(0, |acc, &x| acc + x));
                    batch = Vec::new();
                }
            }
        }

        if batch.len() > 0 {
            sums.push(batch.iter().fold(0, |acc, &x| acc + x));
        }
    } else {
        eprintln!("Error opening file: {}", file_path);
    }

    let max = sums
        .iter()
        .fold(u32::MIN, |current_max, &x| current_max.max(x));

    let top3: Vec<u32> = sums.iter().fold(vec![], |mut acc, &x| {
        if acc.len() < 3 {
            acc.push(x);
            acc.sort_by(|a, b| b.cmp(a)); // Sort in descending order
        } else if x > *acc.last().unwrap() {
            acc.pop();
            acc.push(x);
            acc.sort_by(|a, b| b.cmp(a)); // Sort in descending order
        }
        acc
    });

    (max, top3.iter().fold(0, |acc, &x| acc + x))
}
