// Idea development

pub fn normalizetags(s: String) -> String {
    let mut ret = s.to_owned();
    let tags = vec!["p", "b", "i", "strong", "em", "h1", "h2", "h3", "h4", "h5", "h6", "ul", "ol", "li", "pre", "code", "head", "body", "script", "style"];

    for tag in tags {
        // works only for naked tags
        ret = ret.replace(&format!("<{}>",  tag ).to_ascii_uppercase(), &format!("<{}>",  tag ));
        ret = ret.replace(&format!("</{}>",  tag ).to_ascii_uppercase(), &format!("</{}>",  tag ));
    }
    println!("{}", ret);
    ret
}

// Initial naïve implementation
pub fn closetags(s: String) -> String {
    let mut ret = s.to_owned();
    let mut lines: Vec<String> = ret.split('\n').filter(|&s| !s.is_empty()).map(|x| x.to_owned()).collect();

    let mut newlines: Vec<String> = Vec::new();

    let tags = vec!["p", "b", "i", "strong", "em", "h1", "h2", "h3", "h4", "h5", "h6"];

    for refline in &lines {
        let mut line = refline.to_owned();
        for tag in &tags {
            // bail out if tag is not present globally
            if !s.contains(&format!("<{}>",  tag )) {
                continue;
            }

            // bail out if tag is not present in the line
            if !line.contains(&format!("<{}>",  tag )) {
                continue;
            }

            let open: Vec<_> = line.match_indices(&format!("<{}>",  tag )).collect();
            let close: Vec<_> = line.match_indices(&format!("</{}>",  tag )).collect();
            let diff = open.len() - close.len();

            let mut i = 0;
            while i < diff {
                line.push_str(&format!("</{}>",  tag ));
                i = i + 1;
            };
        }
        newlines.push(line);
    }
    newlines.join("\n")
}

pub fn converttags(s: String) -> String {
    let mut ret = s.to_owned();
    let tags = vec![("b", "strong"), ("i", "em")];

    for tag in tags {
        ret = ret.replace(&format!("<{}>",  tag.0 ), &format!("<{}>",  tag.1 ));
        ret = ret.replace(&format!("</{}>",  tag.0 ), &format!("</{}>",  tag.1 ));
    }
    ret
}

#[cfg(test)]
mod tests {
    use crate::dev::{closetags, normalizetags, converttags};


    #[test]
    fn normalize_test_1 () {
        let ts = "this is a test".to_string();
        assert_eq!("this is a test", normalizetags(ts));
    }

    #[test]
    fn normalize_test_2 () {
        let ts = "<P>this is a test".to_string();
        assert_eq!(normalizetags(ts), "<p>this is a test");
    }

    #[test]
    fn closetags_test_1 () {
        let ts = "this is a test".to_string();
        assert_eq!("this is a test", closetags(ts));
    }

    #[test]
    fn closetags_test_2 () {
        let ts = "<p>this is a test".to_string();
        assert_eq!("<p>this is a test</p>", closetags(ts));
    }

    #[test]
    fn closetags_test_3 () {
        let ts = "<p>this is a test\n<p>this is a test".to_string();
        assert_eq!("<p>this is a test</p>\n<p>this is a test</p>", closetags(ts));
    }

    #[test]
    fn normalizetags_closetags_test_1 () {
        let ts = "<P>this is a test".to_string();
        assert_eq!("<p>this is a test</p>", closetags(normalizetags(ts)));
    }

    #[test]
    fn converttags_test_1 () {
        let ts = "<b>this is a test".to_string();
        assert_eq!("<strong>this is a test", converttags(ts));
    }

    #[test]
    fn normalizetags_closetags_converttags_test_1 () {
        let ts = "<B>this is a test".to_string();
        assert_eq!("<strong>this is a test</strong>", closetags(converttags(normalizetags(ts))));
    }

}
