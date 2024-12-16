use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

pub fn matches_email_regex(string: &str) -> bool {
    Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]{2,}$")
        .unwrap()
        .is_match(string)
}

pub fn are_words_ordered(string: &str) -> bool {
    string
        .unicode_words()
        .collect::<Vec<&str>>()
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<String>>()
        .windows(2)
        .all(|w| w[0] <= w[1])
}

#[cfg(test)]
mod test {
    use super::{are_words_ordered, matches_email_regex};

    #[test]
    fn test_words_ordered() {
        assert!(are_words_ordered("deez nuts"))
    }

    #[test]
    fn test_words_ordered_mixed_case() {
        assert!(are_words_ordered("Deez Nuts zigma"))
    }

    #[test]
    fn test_single_word_is_ordered() {
        assert!(are_words_ordered("deez"))
    }

    #[test]
    fn test_no_word_is_ordered() {
        assert!(are_words_ordered(""))
    }

    #[test]
    fn test_words_unordered() {
        assert!(!are_words_ordered("nuts deez"))
    }

    #[test]
    fn test_words_unordered_mixed_case() {
        assert!(!are_words_ordered("Zigma nuts Deez"))
    }

    #[test]
    fn test_actual_email_address() {
        let actual = matches_email_regex("user@domain.com");
        assert!(actual);
    }

    #[test]
    fn test_email_address_empty() {
        let actual = matches_email_regex("");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_starts_with_spaces() {
        let actual = matches_email_regex(" user@domain.com");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_ends_with_spaces() {
        let actual = matches_email_regex("user@domain.com ");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_missing_domain() {
        let actual = matches_email_regex("user@.com");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_missing_top_level_domain() {
        let actual = matches_email_regex("user@domain");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_missing_user() {
        let actual = matches_email_regex("@domain.com");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_spaces_in_domain() {
        let actual = matches_email_regex("user@ domain.com");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_spaces_in_user() {
        let actual = matches_email_regex("us er@domain.com");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_spaces_in_top_level_domain() {
        let actual = matches_email_regex("user@domain.c om");
        assert!(!actual);
    }

    #[test]
    fn test_email_address_top_level_domain_too_short() {
        let actual = matches_email_regex("user@domain.c");
        assert!(!actual);
    }
}
