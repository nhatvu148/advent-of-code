use advent_of_code::y2022::day_1::process_file;

#[test]
fn test_day_1() {
    let test_cases = [
        ("input/y2022/day_1_1.txt", 24000),
        ("input/y2022/day_1_2.txt", 69177),
    ];

    for (file_path, expected_max) in test_cases.iter() {
        let max = process_file(file_path);

        assert_eq!(max, *expected_max);
    }
}
