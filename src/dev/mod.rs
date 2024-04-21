// Idea development

pub fn normalizetags(s: String) -> String {
    let mut ret = s.to_owned();
    let tags = vec!["p", "b","i","strong","em","h1","h2","h3", "h4", "h5", "h6"];

    for tag in tags {
        ret = ret.replace(&format!("<{}>",  tag ).to_ascii_uppercase(), &format!("<{}>",  tag ));
        ret = ret.replace(&format!("</{}>",  tag ).to_ascii_uppercase(), &format!("</{}>",  tag ));
    }
    println!("{}", ret);
    ret
}

// Initial naïve implementation
pub fn closetags(s: String) -> String {
    let mut ret = s.to_owned();
    let tags = vec!["p", "b","i","strong","em","h1","h2","h3", "h4", "h5", "h6"];

    for tag in tags {
        let open: Vec<_> = s.match_indices(&format!("<{}>",  tag )).collect();
        let close: Vec<_> = s.match_indices(&format!("</{}>",  tag )).collect();
        let diff = open.len() - close.len();
        let mut i = 0;
        while i < diff {
            ret.push_str(&format!("</{}>",  tag ));
            i = i + 1;
        };
    }
    ret
}

#[cfg(test)]
mod tests {
    use crate::dev::{closetags, normalizetags};


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
}
