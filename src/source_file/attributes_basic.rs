use crate::section::section_attributes::SectionAttribute;

pub fn attributes_basic(source: &Option<Vec<SectionAttribute>>) -> Option<String> {
    match source.as_ref() {
        Some(attributes) => {
            let mut response = String::from("");
            attributes.iter().for_each(|x| {
                if let SectionAttribute::Attribute { key, value } = x {
                    response.push_str(
                        format!(
                            r#" {}="{}""#,
                            key.as_ref().unwrap(),
                            value.as_ref().unwrap()
                        )
                        .as_str(),
                    );
                }
            });
            Some(response)
        }
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::section::section_attributes::SectionAttribute;

    #[test]
    fn empty() {
        let input: Option<Vec<SectionAttribute>> = None;
        let expected = None;
        let results = attributes_basic(&input);
        assert_eq!(expected, results);
    }

    #[test]
    fn basic() {
        let input: Option<Vec<SectionAttribute>> = Some(vec![SectionAttribute::Attribute {
            key: Some("id".to_string()),
            value: Some("foxtrot".to_string()),
        }]);
        let expected = Some(r#" id="foxtrot""#.to_string());
        let results = attributes_basic(&input);
        assert_eq!(expected, results);
    }
}
