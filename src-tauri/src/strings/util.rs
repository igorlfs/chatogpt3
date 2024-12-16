use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

const MIN_PASSWORD_LENGTH: usize = 12;

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

pub fn get_password_unsafety_reason(string: &str) -> Option<String> {
    if string.len() < MIN_PASSWORD_LENGTH {
        Some("it's too short".to_string())
    } else if !string.chars().any(|c| c.is_numeric()) {
        Some("it doesn't contain any numbers".to_string())
    } else if !string.chars().any(|c| c.is_alphabetic()) {
        Some("it doesn't contain any letters".to_string())
    } else if !string.chars().any(|c| c.is_uppercase()) {
        Some("it doesn't contain any uppercase letters".to_string())
    } else if !string.chars().any(|c| c.is_ascii_punctuation()) {
        Some("it doesn't contain any recognized special characters".to_string())
    } else if string.to_lowercase().contains("password") {
        Some("it shouldn't contain the word 'password'".to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::{are_words_ordered, get_password_unsafety_reason, matches_email_regex};

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

    #[test]
    fn test_password_is_correct() {
        let reason = get_password_unsafety_reason("abcDef01234@");
        assert!(reason.is_none());
    }

    #[test]
    fn test_password_is_short() {
        let reason = get_password_unsafety_reason("abc").unwrap();
        assert!(reason.contains("short"));
    }

    #[test]
    fn test_password_missing_numbers() {
        let reason = get_password_unsafety_reason("abcdefghijkl").unwrap();
        assert!(reason.contains("numbers"));
    }

    #[test]
    fn test_password_missing_letters() {
        let reason = get_password_unsafety_reason("012345678901").unwrap();
        assert!(reason.contains("letters"));
    }

    #[test]
    fn test_password_missing_uppercase_letters() {
        let reason = get_password_unsafety_reason("0123456789ab").unwrap();
        assert!(reason.contains("uppercase"));
    }

    #[test]
    fn test_password_missing_special_characters() {
        let reason = get_password_unsafety_reason("0123456Abcdef").unwrap();
        assert!(reason.contains("special"));
    }

    #[test]
    fn test_password_should_not_contain_string_password() {
        let reason = get_password_unsafety_reason("password123P@").unwrap();
        assert!(reason.contains("password"));
    }

    #[test]
    fn test_password_should_not_contain_string_password_any_case() {
        let reason = get_password_unsafety_reason("PassWord123P@").unwrap();
        assert!(reason.contains("password"));
    }
}
