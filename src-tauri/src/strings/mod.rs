mod util;

use unicode_segmentation::UnicodeSegmentation;
use util::{are_words_ordered, matches_email_regex};

pub fn alternate_string_case(string: &str) -> String {
    let graphemes = string.graphemes(true).collect::<Vec<&str>>();
    graphemes
        .iter()
        .enumerate()
        .map(|(idx, g)| {
            if idx % 2 == 0 {
                g.to_lowercase()
            } else {
                g.to_uppercase()
            }
        })
        .collect()
}

pub fn match_email_address(string: &str) -> String {
    if matches_email_regex(string) {
        format!("{string} 📩✅")
    } else {
        format!("{string} 📩❎")
    }
}

pub fn is_string_ordered(string: &str) -> String {
    if are_words_ordered(string) {
        format!("Woah, would you look at that: the words in {string} are sorted alphabetically!")
    } else {
        format!("The words in {string} are unsorted. Silly you.")
    }
}

#[cfg(test)]
mod test {
    use super::{alternate_string_case, is_string_ordered, match_email_address};

    #[test]
    fn test_alternate_case_basic_string() {
        let string = alternate_string_case("simple");
        assert_eq!(string, "sImPlE");
    }

    #[test]
    fn test_alternate_case_with_spaces() {
        let string = alternate_string_case("hi sir");
        assert_eq!(string, "hI SiR");
    }

    #[test]
    fn test_alternate_case_with_special_characters() {
        let string = alternate_string_case("têstãç");
        assert_eq!(string, "tÊsTãÇ");
    }

    #[test]
    fn test_alternate_case_with_emojis() {
        let string = alternate_string_case("🤓");
        assert_eq!(string, "🤓");
    }

    #[test]
    fn test_sorted_string() {
        let is_sorted = is_string_ordered("abc");
        assert!(is_sorted.contains("are sorted"));
    }

    #[test]
    fn test_unsorted_string() {
        let is_sorted = is_string_ordered("foo bar");
        assert!(is_sorted.contains("are unsorted"));
    }

    #[test]
    fn test_match_non_email() {
        let is_email = match_email_address("foo");
        assert!(is_email.contains('❎'));
    }

    #[test]
    fn test_match_email() {
        let is_email = match_email_address("foo@bar.com");
        assert!(is_email.contains('✅'));
    }
}
