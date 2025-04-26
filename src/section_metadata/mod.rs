use crate::config::Config;
use crate::section_attr::raw_section_attr;
use crate::section_flag::raw_section_flag;
use crate::section_parent::SectionParent;
use crate::span::Span;
use nom::Parser;
use nom::multi::many0;
use nom::{IResult, branch::alt};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum RawSectionMetaData {
    Attribtue { key: String, spans: Vec<Span> },
    Flag { string: String },
}

pub fn section_metadata<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, (BTreeMap<String, Vec<Vec<Span>>>, Vec<String>)> {
    let (source, raw_metadata) = many0(alt((
        |src| raw_section_flag(src, config, parent, debug),
        |src| raw_section_attr(src, config, parent, debug),
    )))
    .parse(source)?;
    let mut attributes: BTreeMap<String, Vec<Vec<Span>>> = BTreeMap::new();
    raw_metadata.iter().for_each(|metadata| match metadata {
        RawSectionMetaData::Attribtue { key, spans } => {
            match attributes.get_mut(key) {
                Some(payload) => payload.push(spans.clone()),
                None => {
                    attributes.insert(key.to_string(), vec![spans.clone()]);
                    ()
                }
            }
        }
        _ => (),
    });

    let flags = raw_metadata
        .iter()
        .filter_map(|metadata| match metadata {
            RawSectionMetaData::Flag { string } => Some(string.clone()),
            _ => None,
        })
        .collect::<Vec<String>>();
    Ok((source, (attributes, flags)))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn section_metadata_flag_test() {
        let config = &Config::default();
        let source = "-- test-flag\n\n";
        let parent = &SectionParent::Basic;
        let debug = false;
        let left = (BTreeMap::new(), vec!["test-flag".to_string()]);
        let right = section_metadata(source, config, parent, debug).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn section_metadata_flag_whitespace_test() {
        let config = &Config::default();
        let source = "--      foxtrot-bravo     ";
        let parent = &SectionParent::Basic;
        let debug = false;
        let left = (BTreeMap::new(), vec!["foxtrot-bravo".to_string()]);
        let right = section_metadata(source, config, parent, debug).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn section_metadata_attribute_test() {
        let config = &Config::default();
        let source = "-- alfa: bravo\n\n";
        let parent = &SectionParent::Basic;
        let debug = false;
        let mut attributes: BTreeMap<String, Vec<Vec<Span>>> = BTreeMap::new();
        attributes.insert(
            "alfa".to_string(),
            vec![vec![Span::Text {
                content: "bravo".to_string(),
            }]],
        );
        let left = (attributes, vec![]);
        let right = section_metadata(source, config, parent, debug).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn section_metadata_attribute_whitespace_test() {
        let config = &Config::default();
        let source = "--    hotel:      whiskey     \n\n";
        let parent = &SectionParent::Basic;
        let debug = false;
        let mut attributes: BTreeMap<String, Vec<Vec<Span>>> = BTreeMap::new();
        attributes.insert(
            "hotel".to_string(),
            vec![vec![Span::Text {
                content: "whiskey".to_string(),
            }]],
        );
        let left = (attributes, vec![]);
        let right = section_metadata(source, config, parent, debug).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn multiple_metadata_test() {
        let config = &Config::default();
        let source = "-- delta: alfa\n-- foxtrot\n-- delta: bravo\n-- echo";
        let parent = &SectionParent::Basic;
        let debug = false;
        let mut attributes: BTreeMap<String, Vec<Vec<Span>>> = BTreeMap::new();
        attributes.insert(
            "delta".to_string(),
            vec![
                vec![Span::Text {
                    content: "alfa".to_string(),
                }],
                vec![Span::Text {
                    content: "bravo".to_string(),
                }],
            ],
        );
        let left =
            (attributes, vec!["foxtrot".to_string(), "echo".to_string()]);
        let right = section_metadata(source, config, parent, debug).unwrap().1;
        assert_eq!(left, right);
    }
}
