use advent_of_code_2023::days::{
    day_1::*,
    day_2::{
        calculate_power_max_set, is_game_possible, read_games_from_file,
        CubeCounts,
    },
    day_3::{
        calculate_sum, calculate_sum_of_gear_ratios, check_surroundings,
        process_file as process_file_3,
    },
    day_4::{find_matching_card, process_file as process_file_4},
    day_5::{find_lowest_location, process_file as process_file_5},
    day_6::{
        count_number_of_ways_to_beat_record, process_file as process_file_6,
    },
    day_7::{calculate_total, process_file as process_file_7},
    day_8::{find_steps_lcm, process_file as process_file_8, traverse_graph},
    day_9::process_file as process_file_9,
};

#[ignore]
#[test]
fn test_day_1() {
    let test_cases = [
        ("input/day_1_1.txt", 142),
        ("input/day_1_2.txt", 54087),
        ("input/day_1_3.txt", 281),
    ];

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
    let test_cases = [
        ("input/day_3_1.txt", 4361, 467835),
        ("input/day_3_2.txt", 527446, 73201705),
    ];

    for (file_path, expected_sum, expected_sum_gear_ratios) in test_cases.iter()
    {
        match process_file_3(file_path) {
            Ok((part_numbers, lines)) => {
                // for (i, part_number) in part_numbers.iter().enumerate() {
                //     println!("Row {}: {:?}", i + 1, part_number);
                // }
                let (result_parts, gears) =
                    check_surroundings(&part_numbers, &lines);

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
    let test_cases = [
        ("input/day_4_1.txt", 13, 30),
        ("input/day_4_2.txt", 27454, 6857330),
    ];

    for (file_path, expected_sum, expected_total_scratch_cards) in
        test_cases.iter()
    {
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

#[ignore]
#[test]
fn test_day_5() {
    let test_cases: [(&str, u128, u128); 2] = [
        ("input/day_5_1.txt", 35, 46),
        ("input/day_5_2.txt", 484023871, 46294175),
    ];

    for (file_path, expected_lowest, expected_lowest_for_seed_pairs) in
        test_cases.iter()
    {
        match process_file_5(file_path) {
            Ok(map_data) => {
                let (min_location, min_location_for_seed_pairs) =
                    find_lowest_location(&map_data);

                assert_eq!(min_location, *expected_lowest);
                assert_eq!(
                    min_location_for_seed_pairs,
                    *expected_lowest_for_seed_pairs
                );
            }
            Err(err) => eprintln!("Error reading file: {}", err),
        }
    }
}

#[ignore]
#[test]
fn test_day_6() {
    let test_cases: [(&str, u128, u128); 2] = [
        ("input/day_6_1.txt", 288, 71503),
        ("input/day_6_2.txt", 861300, 28101347),
    ];

    for (file_path, expected_products, expected_combined_products) in
        test_cases.iter()
    {
        match process_file_6(file_path) {
            Ok((time_vector, distance_vector)) => {
                let (products, combined_products) =
                    count_number_of_ways_to_beat_record(
                        &time_vector,
                        &distance_vector,
                    );

                assert_eq!(products, *expected_products);
                assert_eq!(combined_products, *expected_combined_products);
            }
            Err(err) => eprintln!("Error reading file: {}", err),
        }
    }
}

#[ignore]
#[test]
fn test_day_7() {
    let test_cases: [(&str, u32, u32); 2] = [
        ("input/day_7_1.txt", 6440, 5905),
        ("input/day_7_2.txt", 250951660, 251481660),
    ];

    for (file_path, expected_total, expected_joker_total) in test_cases.iter() {
        let result_map = process_file_7(file_path);
        let (total, joker_total) = calculate_total(&result_map);

        assert_eq!(total, *expected_total);
        assert_eq!(joker_total, *expected_joker_total);
    }
}

#[ignore]
#[test]
fn test_day_8() {
    let test_cases: [(&str, u64, u64); 3] = [
        ("input/day_8_1.txt", 2, 2),
        ("input/day_8_2.txt", 19099, 17099847107071),
        ("input/day_8_3.txt", 0, 6),
    ];

    for (file_path, expected_steps, expected_steps_lcm) in test_cases.iter() {
        match process_file_8(file_path) {
            Ok(data) => {
                let steps = traverse_graph(&data.graph, &data.instructions);

                let steps_lcm = find_steps_lcm(&data.graph, &data.instructions);

                assert_eq!(steps, *expected_steps);
                assert_eq!(steps_lcm, *expected_steps_lcm);
            }
            Err(err) => eprintln!("Error reading file: {}", err),
        }
    }
}

#[test]
fn test_day_9() {
    let test_cases: [(&str, i32, i32); 2] = [
        ("input/day_9_1.txt", 114, 0),
        ("input/day_9_2.txt", 1584748274, 0),
    ];

    for (file_path, expected_sums, _) in test_cases.iter() {
        let sums = process_file_9(file_path);

        assert_eq!(sums, *expected_sums);
    }
}
