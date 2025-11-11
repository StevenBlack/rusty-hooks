use std::collections::HashSet;

/// Utility functions for rusty-hooks.
pub fn close_tags(s: String) -> String {
    use regex::Regex;
    let allowed_tags: HashSet<&str> = [
        "p",
        "blockquote",
        "li",
        "ul",
        "ol",
        "strong",
        "em",
        "b",
        "i",
        // add more tags as needed
    ].iter().cloned().collect();

    let tag_re = Regex::new(r"(?i)<(/?)([a-zA-Z0-9]+)[^>]*>").unwrap();
    let mut open_tags = Vec::new();

    for cap in tag_re.captures_iter(&s) {
        let is_closing = &cap[1] == "/";
        let tag_name = cap[2].to_lowercase();

        if allowed_tags.contains(tag_name.as_str()) {
            if !is_closing {
                open_tags.push(tag_name.clone());
            } else {
                if let Some(pos) = open_tags.iter().rposition(|t| t == &tag_name) {
                    open_tags.remove(pos);
                }
            }
        }
    }

    let mut result = s.clone();
    for tag in open_tags.iter().rev() {
        result.push_str(&format!("</{}>", tag));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(
            close_tags("<strong>Hello World".to_string()),
            "<strong>Hello World</strong>"
        );
    }

    #[test]
    fn test_nested() {
        assert_eq!(
            close_tags("The <strong>rain in <em>Spain stays mainly on the plane".to_string()),
            "The <strong>rain in <em>Spain stays mainly on the plane</em></strong>"
        );
    }

    #[test]
    fn test_closed_tags() {
        assert_eq!(
            close_tags("<strong>Hello</strong> World".to_string()),
            "<strong>Hello</strong> World"
        );
    }

    #[test]
    fn test_multiple_open_tags() {
        assert_eq!(
            close_tags("<ul><li>Test".to_string()),
            "<ul><li>Test</li></ul>"
        );
    }

    #[test]
    fn test_self_closing_tags() {
        assert_eq!(
            close_tags("<ul><br/>Hello".to_string()),
            "<ul><br/>Hello</ul>"
        );
    }

    #[test]
    fn test_attributes() {
        assert_eq!(
            close_tags("<blockquote cite='foo'>Quote".to_string()),
            "<blockquote cite='foo'>Quote</blockquote>"
        );
    }

    #[test]
    fn test_mixed_case_tags() {
        assert_eq!(
            close_tags("<UL><LI>Text".to_string()),
            "<UL><LI>Text</LI></UL>"
        );
    }

    #[test]
    fn test_no_tags() {
        assert_eq!(
            close_tags("Just text".to_string()),
            "Just text"
        );
    }

    #[test]
    fn test_ignored_tags() {
        // <div> is not in the allowed list, so it won't be closed
        assert_eq!(
            close_tags("<div><strong>Test".to_string()),
            "<div><strong>Test</strong>"
        );
    }
}
