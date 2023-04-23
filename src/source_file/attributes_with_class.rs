use crate::section::section_attributes::SectionAttribute;

pub fn attributes_with_class(
    source: &Option<Vec<SectionAttribute>>,
    class_name: &str,
) -> Option<String> {
    let mut class_string = format!(r#" class="{}"#, class_name);
    let mut response = String::from("");
    match source {
        Some(v) => {
            v.iter().for_each(|x| match x {
                SectionAttribute::Attribute { key, value } => {
                    if key.as_ref().unwrap() == "class" {
                        class_string.push_str(" ");
                        class_string.push_str(value.as_ref().unwrap().as_str());
                    } else {
                        response.push_str(
                            format!(
                                r#" {}="{}""#,
                                key.as_ref().unwrap(),
                                value.as_ref().unwrap()
                            )
                            .as_str(),
                        );
                    }
                }
            });
        }
        _ => (),
    }
    class_string.push_str(r#"""#);
    response.push_str(class_string.as_str());
    Some(response)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::section::section_attributes::SectionAttribute;

    #[test]
    fn no_attributes_just_class() {
        let input: Option<Vec<SectionAttribute>> = None;
        let expected: Option<String> = Some(String::from(r#" class="alfa""#));
        let results = attributes_with_class(&input, "alfa");
        assert_eq!(expected, results);
    }

    #[test]
    fn attributes_without_extra_class() {
        let input: Option<Vec<SectionAttribute>> = Some(vec![SectionAttribute::Attribute {
            key: Some("id".to_string()),
            value: Some("bravo".to_string()),
        }]);
        let expected = Some(r#" id="bravo" class="charlie""#.to_string());
        let results = attributes_with_class(&input, "charlie");
        assert_eq!(expected, results);
    }

    #[test]
    fn class_in_attributes() {
        let input: Option<Vec<SectionAttribute>> = Some(vec![
            SectionAttribute::Attribute {
                key: Some("id".to_string()),
                value: Some("delta".to_string()),
            },
            SectionAttribute::Attribute {
                key: Some("class".to_string()),
                value: Some("echo".to_string()),
            },
        ]);
        let expected = Some(r#" id="delta" class="foxtrot echo""#.to_string());
        let results = attributes_with_class(&input, "foxtrot");
        assert_eq!(expected, results);
    }
}
