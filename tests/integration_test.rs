use advent_of_code_2023::days::day_1::*;

#[test]
fn test_day_1() {
    let file_path_1 = "input/day_1_1.txt";
    match process_file(file_path_1) {
        Ok(total) => {
            assert_eq!(total, 142);
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }

    let file_path_2 = "input/day_1_2.txt";
    match process_file(file_path_2) {
        Ok(total) => {
            assert_eq!(total, 54708);
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}
