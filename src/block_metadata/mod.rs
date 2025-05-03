pub mod attr;
pub mod bound;
pub mod flag;
pub mod parent;

use crate::block_metadata::attr::raw_block_attr;
use crate::block_metadata::flag::raw_block_flag;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span::Span;
use nom::Parser;
use nom::multi::many0;
use nom::{IResult, branch::alt};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum RawBlockMetaData {
    Attribute { key: String, spans: Vec<Span> },
    Flag { string: String },
    // Attrs {
    //     collection: Vec<RawBlockMetaData::Attribute>,
    // },
}

pub fn block_metadata_dev<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
) -> IResult<&'a str, (Vec<RawBlockMetaData>, Vec<RawBlockMetaData>)> {
    let (source, raw_metadata) = many0(alt((
        |src| raw_block_attr(src, config, parent),
        |src| raw_block_flag(src, config, parent),
    )))
    .parse(source)?;
    let attrs = raw_metadata
        .iter()
        .filter_map(|metadata| match metadata {
            RawBlockMetaData::Attribute { .. } => Some(metadata.clone()),
            _ => None,
        })
        .collect();
    let flags = raw_metadata
        .iter()
        .filter_map(|metadata| match metadata {
            RawBlockMetaData::Flag { .. } => Some(metadata.clone()),
            _ => None,
        })
        .collect();
    Ok((source, (attrs, flags)))
}

pub fn block_metadata<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
) -> IResult<&'a str, (BTreeMap<String, Vec<Span>>, Vec<String>)> {
    let (source, raw_metadata) = many0(alt((
        |src| raw_block_attr(src, config, parent),
        |src| raw_block_flag(src, config, parent),
    )))
    .parse(source)?;
    let mut attrs: BTreeMap<String, Vec<Span>> = BTreeMap::new();
    raw_metadata.iter().for_each(|metadata| {
        if let RawBlockMetaData::Attribute { key, spans } = metadata {
            match attrs.get_mut(key) {
                Some(payload) => spans.iter().for_each(|span| {
                    payload.push(span.clone());
                }),
                None => {
                    attrs.insert(key.to_string(), spans.clone());
                }
            }
        }
    });
    let flags = raw_metadata
        .iter()
        .filter_map(|metadata| match metadata {
            RawBlockMetaData::Flag { string } => Some(string.clone()),
            _ => None,
        })
        .collect::<Vec<String>>();
    Ok((source, (attrs, flags)))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn block_metadata_flag_test() {
        let config = &Config::default();
        let source = "-- test-flag\n\n";
        let parent = &BlockParent::Basic;
        let left = (BTreeMap::new(), vec!["test-flag".to_string()]);
        let right = block_metadata(source, config, parent).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn block_metadata_flag_whitespace_test() {
        let config = &Config::default();
        let source = "--      foxtrot-bravo     ";
        let parent = &BlockParent::Basic;
        let left = (BTreeMap::new(), vec!["foxtrot-bravo".to_string()]);
        let right = block_metadata(source, config, parent).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn block_metadata_attribute_test() {
        let config = &Config::default();
        let source = "-- alfa: bravo\n\n";
        let parent = &BlockParent::Basic;
        let mut attributes: BTreeMap<String, Vec<Span>> = BTreeMap::new();
        attributes.insert(
            "alfa".to_string(),
            vec![Span::Text {
                content: "bravo".to_string(),
            }],
        );
        let left = (attributes, vec![]);
        let right = block_metadata(source, config, parent).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn block_metadata_attribute_whitespace_test() {
        let config = &Config::default();
        let source = "--    hotel:      whiskey     \n\n";
        let parent = &BlockParent::Basic;
        let mut attributes: BTreeMap<String, Vec<Span>> = BTreeMap::new();
        attributes.insert(
            "hotel".to_string(),
            vec![Span::Text {
                content: "whiskey".to_string(),
            }],
        );
        let left = (attributes, vec![]);
        let right = block_metadata(source, config, parent).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn multiple_metadata_test() {
        let config = &Config::default();
        let source = "-- delta: alfa\n-- foxtrot\n-- delta: bravo\n-- echo";
        let parent = &BlockParent::Basic;
        let mut attributes: BTreeMap<String, Vec<Span>> = BTreeMap::new();
        attributes.insert(
            "delta".to_string(),
            vec![
                Span::Text {
                    content: "alfa".to_string(),
                },
                Span::Text {
                    content: "bravo".to_string(),
                },
            ],
        );
        let left =
            (attributes, vec!["foxtrot".to_string(), "echo".to_string()]);
        let right = block_metadata(source, config, parent).unwrap().1;
        assert_eq!(left, right);
    }
}
