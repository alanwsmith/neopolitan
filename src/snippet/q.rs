use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn q(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<q");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</q>");
    Snippet::QuotationTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::q::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::QuotationTag {
            string: Some("<q>Set the piece</q>".to_string()),
        };
        let results = q("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::QuotationTag {
            string: Some(r#"<q class="alfa">Pile the coal</q>"#.to_string()),
        };
        let results = q("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::QuotationTag {
            string: Some(
                r#"<q id="echo" class="delta foxtrot">Raise the sail</q>"#.to_string(),
            ),
        };
        let results = q("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::QuotationTag {
            string: Some(r#"<q checked>Lift the stone</q>"#.to_string()),
        };
        let results = q("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
