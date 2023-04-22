use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet::Snippet;

pub fn samp(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<samp");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</samp>");
    Snippet::SampleOutputTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::samp::*;
    use crate::snippet::snippet::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::SampleOutputTag {
            string: Some("<samp>Set the piece</samp>".to_string()),
        };
        let results = samp("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::SampleOutputTag {
            string: Some(r#"<samp class="alfa">Pile the coal</samp>"#.to_string()),
        };
        let results = samp("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::SampleOutputTag {
            string: Some(
                r#"<samp id="echo" class="delta foxtrot">Raise the sail</samp>"#.to_string(),
            ),
        };
        let results = samp("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::SampleOutputTag {
            string: Some(r#"<samp checked>Lift the stone</samp>"#.to_string()),
        };
        let results = samp("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
