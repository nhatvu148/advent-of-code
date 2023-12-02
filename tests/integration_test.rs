use advent_of_code_2023::days::{
    day_1::*,
    day_2::{is_game_possible, read_games_from_file, CubeCounts},
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

#[test]
fn test_day_2() {
    // Define a map of file paths and their expected sums
    let test_cases = [("input/day_2_1.txt", 8), ("input/day_2_2.txt", 2679)];
    let cube_counts: CubeCounts = [
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]
    .iter()
    .cloned()
    .collect();

    // Iterate through the test cases
    for (file_path, expected_sum) in test_cases.iter() {
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
            }
            Err(err) => eprintln!("Error reading file: {}", err),
        }
    }
}
