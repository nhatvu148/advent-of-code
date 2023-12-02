use advent_of_code_2023::days::day_1::*;

#[test]
fn test_day_1() {
    // Define a map of file paths and their expected totals
    let test_cases = [
        ("input/day_1_1.txt", 142),
        ("input/day_1_2.txt", 54087),
        ("input/day_1_3.txt", 281),
    ];

    // Iterate through the test cases
    for (file_path, expected_total) in test_cases.iter() {
        match process_file(file_path) {
            Ok(total) => {
                assert_eq!(total, *expected_total);
            }
            Err(err) => eprintln!("Error reading file {}: {}", file_path, err),
        }
    }
}
