use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;
use html_escape;

pub fn dfn(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<dfn");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    let escaped_text = html_escape::encode_text(text).to_string();
    response.push_str(escaped_text.as_str());
    response.push_str("</dfn>");
    Snippet::DefinitionTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::snippets::dfn::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::DefinitionTag {
            string: Some("<dfn>Set the piece</dfn>".to_string()),
        };
        let results = dfn("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::DefinitionTag {
            string: Some(r#"<dfn class="alfa">Pile the coal</dfn>"#.to_string()),
        };
        let results = dfn("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::DefinitionTag {
            string: Some(
                r#"<dfn id="echo" class="delta foxtrot">Raise the sail</dfn>"#.to_string(),
            ),
        };
        let results = dfn("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::DefinitionTag {
            string: Some(r#"<dfn checked>Lift the stone</dfn>"#.to_string()),
        };
        let results = dfn("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
