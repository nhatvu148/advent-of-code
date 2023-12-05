use advent_of_code_2023::days::{
    day_1::*,
    day_2::{calculate_power_max_set, is_game_possible, read_games_from_file, CubeCounts},
    day_3::{
        calculate_sum, calculate_sum_of_gear_ratios, check_surroundings,
        process_file as process_file_3,
    },
    day_4::{find_matching_card, process_file as process_file_4},
    day_5::{find_lowest_location, process_file as process_file_5},
};

#[ignore]
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

#[ignore]
#[test]
fn test_day_2() {
    // Define a map of file paths and their expected sums
    let test_cases = [
        ("input/day_2_1.txt", 8, 2286),
        ("input/day_2_2.txt", 2679, 77607),
    ];
    let cube_counts: CubeCounts = [
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]
    .iter()
    .cloned()
    .collect();

    // Iterate through the test cases
    for (file_path, expected_sum, expected_power_max) in test_cases.iter() {
        match read_games_from_file(file_path) {
            Ok(games) => {
                // for (i, game) in games.iter().enumerate() {
                //     println!("Game {}: {:?}", i + 1, game);
                // }

                let possible_games: Vec<usize> = games
                    .iter()
                    .filter(|&game| is_game_possible(game, &cube_counts))
                    .map(|game| game.id)
                    .collect();

                let sum: usize = possible_games.iter().sum();
                println!("Possible games: {:?}", possible_games);
                println!("Sum of IDs: {}", sum);
                assert_eq!(sum, *expected_sum);

                let power_max = calculate_power_max_set(&games);
                println!("Power max cubes: {}", power_max);
                assert_eq!(power_max, *expected_power_max);
            }
            Err(err) => eprintln!("Error reading file: {}", err),
        }
    }
}

#[ignore]
#[test]
fn test_day_3() {
    // Define a map of file paths and their expected sums
    let test_cases = [
        ("input/day_3_1.txt", 4361, 467835),
        ("input/day_3_2.txt", 527446, 73201705),
    ];

    // Iterate through the test cases
    for (file_path, expected_sum, expected_sum_gear_ratios) in test_cases.iter() {
        match process_file_3(file_path) {
            Ok((part_numbers, lines)) => {
                // for (i, part_number) in part_numbers.iter().enumerate() {
                //     println!("Row {}: {:?}", i + 1, part_number);
                // }
                let (result_parts, gears) = check_surroundings(&part_numbers, &lines);

                let sum = calculate_sum(&result_parts);
                let sum_gear_ratios = calculate_sum_of_gear_ratios(&gears);

                assert_eq!(sum, *expected_sum);
                assert_eq!(sum_gear_ratios, *expected_sum_gear_ratios);
            }
            Err(err) => eprintln!("Error reading file: {}", err),
        }
    }
}

#[ignore]
#[test]
fn test_day_4() {
    // Define a map of file paths and their expected sums
    let test_cases = [
        ("input/day_4_1.txt", 13, 30),
        ("input/day_4_2.txt", 27454, 6857330),
    ];

    // Iterate through the test cases
    for (file_path, expected_sum, expected_total_scratch_cards) in test_cases.iter() {
        match process_file_4(file_path) {
            Ok((winning_numbers, having_numbers)) => {
                let (sum, total_scratch_cards) =
                    find_matching_card(&winning_numbers, &having_numbers);

                assert_eq!(sum, *expected_sum);
                assert_eq!(total_scratch_cards, *expected_total_scratch_cards);
            }
            Err(err) => eprintln!("Error reading file: {}", err),
        }
    }
}

#[test]
fn test_day_5() {
    // Define a map of file paths and their expected sums
    let test_cases: [(&str, u128); 2] =
        [("input/day_5_1.txt", 35), ("input/day_5_2.txt", 484023871)];

    // Iterate through the test cases
    for (file_path, expected_lowest) in test_cases.iter() {
        match process_file_5(file_path) {
            Ok(map_data) => {
                let lowest = find_lowest_location(&map_data);

                assert_eq!(lowest, *expected_lowest);
            }
            Err(err) => eprintln!("Error reading file: {}", err),
        }
    }
}
