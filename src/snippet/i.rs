use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn i(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<i");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</i>");
    Snippet::IdiomaticTextTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::i::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::IdiomaticTextTag {
            string: Some("<i>Set the piece</i>".to_string()),
        };
        let results = i("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::IdiomaticTextTag {
            string: Some(r#"<i class="alfa">Pile the coal</i>"#.to_string()),
        };
        let results = i("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::IdiomaticTextTag {
            string: Some(
                r#"<i id="echo" class="delta foxtrot">Raise the sail</i>"#.to_string(),
            ),
        };
        let results = i("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::IdiomaticTextTag {
            string: Some(r#"<i checked>Lift the stone</i>"#.to_string()),
        };
        let results = i("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
