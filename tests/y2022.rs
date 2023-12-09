use advent_of_code::y2022::day_1::process_file;

#[test]
fn test_day_1() {
    let test_cases = [
        ("input/y2022/day_1_1.txt", 24000, 45000),
        ("input/y2022/day_1_2.txt", 69177, 207456),
    ];

    for (file_path, expected_max, expected_sum_top_3) in test_cases.iter() {
        let (max, sum_top_3) = process_file(file_path);

        assert_eq!(max, *expected_max);
        assert_eq!(sum_top_3, *expected_sum_top_3);
    }
}
