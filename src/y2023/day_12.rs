use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};

fn generate_cases(
    chars: &mut Vec<char>,
    index: usize,
    results: &mut Vec<String>,
) {
    if index == chars.len() {
        let result_str: String = chars.iter().collect();
        if !results.contains(&result_str) {
            results.push(result_str);
        }
        return;
    }

    if chars[index] == '?' {
        chars[index] = '.';
        generate_cases(chars, index + 1, results);
        chars[index] = '#';
        generate_cases(chars, index + 1, results);
        chars[index] = '?'; // Reset back to "?" for backtracking
    } else {
        generate_cases(chars, index + 1, results);
    }
}

fn generate_cases_iterative(chars: &mut Vec<char>, results: &mut Vec<String>) {
    let mut stack: Vec<(usize, Vec<char>)> = Vec::new();

    stack.push((0, chars.clone()));

    while let Some((index, current_chars)) = stack.pop() {
        if index == chars.len() {
            let result_str: String = current_chars.iter().collect();
            if !results.contains(&result_str) {
                results.push(result_str);
            }
        } else if current_chars[index] == '?' {
            for &replace_char in &['.', '#'] {
                let mut new_chars = current_chars.clone();
                new_chars[index] = replace_char;
                stack.push((index + 1, new_chars));
            }
        } else {
            stack.push((index + 1, current_chars));
        }
    }
}

fn count_damages(input: &str) -> Vec<usize> {
    input
        .split('.')
        .map(|substring| substring.chars().filter(|&c| c == '#').count())
        .filter(|&count| count > 0)
        .collect()
}

fn compare_vectors(vec1: &[usize], vec2: &[usize]) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }

    vec1.iter().zip(vec2.iter()).all(|(&a, &b)| a == b)
}

// fn compare_vectors(vec1: &[usize], vec2: &[usize]) -> bool {
//     vec1.iter().eq(vec2.iter())
// }

fn parse_line(line: &str) -> (String, Vec<usize>) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let records: String = parts[0].to_string();

    let conditions: Vec<usize> = parts[1]
        .split(',')
        .flat_map(|s| s.parse::<usize>())
        .collect();

    (records, conditions)
}

fn unfold(records: &str, conditions: &[usize]) -> (String, Vec<usize>) {
    let unfolded_records: String = std::iter::repeat(records)
        .take(5)
        .collect::<Vec<&str>>()
        .join("?");

    let unfolded_conditions: Vec<usize> = std::iter::repeat(conditions)
        .take(5)
        .flat_map(|condition| condition.iter().cloned())
        .collect();

    (unfolded_records, unfolded_conditions)
}

pub fn process_file(file_path: &str) -> usize {
    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        let sum = reader
            .lines()
            .par_bridge()
            .map(|line| {
                let line = line.unwrap();
                let (records, conditions) = parse_line(&line);
                // let (records, conditions) = unfold(&records, &conditions);
                // println!("Records: {:?}, Conditions: {:?}", records, conditions);

                let mut chars: Vec<char> = records.chars().collect();
                let mut results: Vec<String> = Vec::new();

                // generate_cases(&mut chars, 0, &mut results);
                generate_cases_iterative(&mut chars, &mut results);

                let count = results
                    .par_iter()
                    .filter(|case| {
                        let is_matching =
                            compare_vectors(&count_damages(case), &conditions);
                        if is_matching {
                            println!(
                                "case {:?} matching {:?} ? {}",
                                case, conditions, is_matching
                            );
                        }
                        is_matching
                    })
                    .count();

                println!("count for {:?} : {}", records, count);
                count
            })
            .reduce(|| 0, |a, b| a + b);

        println!("Total sum: {}", sum);
        sum
    } else {
        eprintln!("Error opening file: {}", file_path);
        0
    }
}
