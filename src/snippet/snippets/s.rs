use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn s(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<s");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</s>");
    Snippet::StrikethroughTag  {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::snippets::s::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::StrikethroughTag  {
            string: Some("<s>Set the piece</s>".to_string()),
        };
        let results = s("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::StrikethroughTag  {
            string: Some(r#"<s class="alfa">Pile the coal</s>"#.to_string()),
        };
        let results = s("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::StrikethroughTag  {
            string: Some(
                r#"<s id="echo" class="delta foxtrot">Raise the sail</s>"#.to_string(),
            ),
        };
        let results = s("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::StrikethroughTag  {
            string: Some(r#"<s checked>Lift the stone</s>"#.to_string()),
        };
        let results = s("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
