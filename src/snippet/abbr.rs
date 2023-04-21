use crate::snippet::snippet::Snippet;

pub fn abbr(text: &str, _attributes: &str) -> Snippet {
    let mut response = String::from("<abbr>");
    response.push_str(text);
    response.push_str("</abbr>");
    Snippet::Abbr {
        string: Some(response.to_string()),
    }
}
