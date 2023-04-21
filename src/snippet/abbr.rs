use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet::Snippet;

pub fn abbr(text: &str, _attributes: &str) -> Snippet {
    let mut response = String::from("<abbr>");
    response.push_str(text);
    response.push_str("</abbr>");
    Snippet::Abbr {
        string: Some(response.to_string()),
    }
}

pub fn abbr_dev(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<abbr");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };

    // response.push_str(attributes.unwrap().1.as_str());
    response.push_str(">");

    // class="alfa">"#);
    response.push_str(text);
    response.push_str("</abbr>");
    Snippet::Abbr {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::abbr::*;
    use crate::snippet::snippet::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::Abbr {
            string: Some("<abbr>Set the piece here</abbr>".to_string()),
        };
        let results = abbr("Set the piece here", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::Abbr {
            string: Some(r#"<abbr class="alfa">Pile the coal</abbr>"#.to_string()),
        };
        let results = abbr_dev("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }
}
