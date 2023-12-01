mod tests {
    use crate::days::day_1::*;

    #[test]
    fn test_extract_first_and_last_digits() {
        assert_eq!(extract_first_and_last_digits("123"), Some(13));
        assert_eq!(extract_first_and_last_digits("42abc7"), Some(47));
        assert_eq!(extract_first_and_last_digits("abc"), None);
        assert_eq!(extract_first_and_last_digits(""), None);
    }
}
