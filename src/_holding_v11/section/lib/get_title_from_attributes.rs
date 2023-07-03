use crate::section::section_attributes::SectionAttribute;

pub fn get_title_from_attributes(source: &Option<Vec<SectionAttribute>>) -> Option<String> {
    let mut response: Option<String> = None;
    match source.as_ref() {
        Some(attributes) => {
            attributes.iter().for_each(|x| match x {
                SectionAttribute::Attribute { key, value } => {
                    if key.as_ref().unwrap() == "title" {
                        response = Some(value.as_ref().unwrap().to_string());
                    }
                },
            });
        }
        None => ()
    }
    response
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::section::section_attributes::SectionAttribute;

    #[test]
    fn empty() {
        let input: Option<Vec<SectionAttribute>> = None;
        let expected = None;
        let results = get_title_from_attributes(&input);
        assert_eq!(expected, results);
    }

    #[test]
    fn basic() {
        let input: Option<Vec<SectionAttribute>> = Some(vec![SectionAttribute::Attribute {
            key: Some("title".to_string()),
            value: Some("tango".to_string()),
        }]);
        let expected = Some("tango".to_string());
        let results = get_title_from_attributes(&input);
        assert_eq!(expected, results);
    }
}
