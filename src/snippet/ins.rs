use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn ins(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<ins");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</ins>");
    Snippet::InsertTag   {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::ins::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::InsertTag   {
            string: Some("<ins>Set the piece</ins>".to_string()),
        };
        let results = ins("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::InsertTag   {
            string: Some(r#"<ins class="alfa">Pile the coal</ins>"#.to_string()),
        };
        let results = ins("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::InsertTag   {
            string: Some(
                r#"<ins id="echo" class="delta foxtrot">Raise the sail</ins>"#.to_string(),
            ),
        };
        let results = ins("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::InsertTag   {
            string: Some(r#"<ins checked>Lift the stone</ins>"#.to_string()),
        };
        let results = ins("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
