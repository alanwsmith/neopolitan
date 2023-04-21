use crate::section::section_attributes::SectionAttribute;

pub fn attribute_with_class(source: Option<Vec<SectionAttribute>>) -> Option<String> {
    None
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::section::section_attributes::SectionAttribute;

    #[test]
    fn empty() {
        let input: Option<Vec<SectionAttribute>> = None;
        let expected = None;
        let results = attribute_with_class(input);
        assert_eq!(expected, results);
    }
}
