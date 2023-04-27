use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;
use html_escape;

pub fn code(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<code");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    let escaped_text = html_escape::encode_text(text).to_string();
    response.push_str(escaped_text.as_str());
    response.push_str("</code>");
    Snippet::CodeTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::snippet_enum::Snippet;
    use crate::snippet::snippets::code::*;

    #[test]
    fn basic() {
        let expected = Snippet::CodeTag {
            string: Some("<code>Set the piece</code>".to_string()),
        };
        let results = code("Set the piece", "");
        assert_eq!(expected, results);
    }
}

