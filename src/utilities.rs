/// Utility functions for rusty-hooks.
pub fn close_tags(s: String) -> String {
    use regex::Regex;
    let tag_re = Regex::new(r"(?i)<(/?)([a-zA-Z0-9]+)[^>]*>").unwrap();
    let mut open_tags = Vec::new();

    for cap in tag_re.captures_iter(&s) {
        let is_closing = &cap[1] == "/";
        let tag_name = cap[2].to_lowercase();

        if !is_closing {
            open_tags.push(tag_name.clone());
        } else {
            if let Some(pos) = open_tags.iter().rposition(|t| t == &tag_name) {
                open_tags.remove(pos);
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
            close_tags("<div><span>Test".to_string()),
            "<div><span>Test</span></div>"
        );
    }

    #[test]
    fn test_self_closing_tags() {
        assert_eq!(
            close_tags("<div><br/>Hello".to_string()),
            "<div><br/>Hello</div>"
        );
    }

    #[test]
    fn test_attributes() {
        assert_eq!(
            close_tags("<a href='#'>Link".to_string()),
            "<a href='#'>Link</a>"
        );
    }

    #[test]
    fn test_mixed_case_tags() {
        assert_eq!(
            close_tags("<DIV><Span>Text".to_string()),
            "<DIV><Span>Text</Span></DIV>"
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
    fn test_complex() {
        assert_eq!(
            close_tags("<ul><li>Item 1<li>Item 2".to_string()),
            "<ul><li>Item 1<li>Item 2</li></li></ul>"
        );
    }
}
