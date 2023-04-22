use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn $TAG(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<$TAG");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</$TAG>");
    Snippet::$ENUM {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::snippets::$TAG::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::$ENUM {
            string: Some("<$TAG>Set the piece</$TAG>".to_string()),
        };
        let results = $TAG("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::$ENUM {
            string: Some(r#"<$TAG class="alfa">Pile the coal</$TAG>"#.to_string()),
        };
        let results = $TAG("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::$ENUM {
            string: Some(
                r#"<$TAG id="echo" class="delta foxtrot">Raise the sail</$TAG>"#.to_string(),
            ),
        };
        let results = $TAG("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::$ENUM {
            string: Some(r#"<$TAG checked>Lift the stone</$TAG>"#.to_string()),
        };
        let results = $TAG("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
