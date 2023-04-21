use crate::section::section_attributes::SectionAttribute;

pub fn attributes_with_class_old(
    source: &Option<Vec<SectionAttribute>>,
    _class_name: Option<String>,
) -> Option<String> {
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

pub fn attributes_with_class_dev(
    source: &Option<Vec<SectionAttribute>>,
    class_name: Option<String>,
) -> Option<String> {
    match class_name {
        Some(name) => {
            let response = format!(r#" class="{}""#, name.as_str());
            Some(response)
        }
        None => match source.as_ref() {
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
        },
    }
}

pub fn attributes_with_class_dev_2(
    _source: &Option<Vec<SectionAttribute>>,
    class_name: &str,
) -> Option<String> {
    let mut class_string = format!(r#" class="{}"#, class_name);
    class_string.push_str(r#"""#);
    Some(format!("{}", class_string))
}

pub fn attributes_with_class(
    source: &Option<Vec<SectionAttribute>>,
    class_name: &str,
) -> Option<String> {
    let mut class_string = format!(r#" class="{}"#, class_name);
    let mut response = String::from("");
    match source {
        Some(v) => {
            v.iter().for_each(|x| {
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
        }
        _ => (),
    }
    class_string.push_str(r#"""#);
    response.push_str(class_string.as_str());
    // Some(format!(r#" id="foxtrot"{}"#, class_string))
    Some(response)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::section::section_attributes::SectionAttribute;

    #[test]
    fn no_attributes_just_class() {
        let input: Option<Vec<SectionAttribute>> = None;
        let expected: Option<String> = Some(String::from(r#" class="title""#));
        let results = attributes_with_class(&input, "title");
        assert_eq!(expected, results);
    }

    #[test]
    fn attributes_without_extra_class() {
        let input: Option<Vec<SectionAttribute>> = Some(vec![SectionAttribute::Attribute {
            key: Some("id".to_string()),
            value: Some("foxtrot".to_string()),
        }]);
        let expected = Some(r#" id="foxtrot" class="title""#.to_string());
        let results = attributes_with_class(&input, "title");
        assert_eq!(expected, results);
    }

    #[ignore]
    #[test]
    fn class_in_attributes() {
        let input: Option<Vec<SectionAttribute>> = Some(vec![SectionAttribute::Attribute {
            key: Some("id".to_string()),
            value: Some("foxtrot".to_string()),
        }]);
        let expected = Some(r#" id="foxtrot" class="title""#.to_string());
        let results = attributes_with_class(&input, "title");
        assert_eq!(expected, results);
    }
}
