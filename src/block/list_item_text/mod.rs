#![allow(unused)]
use crate::block::Block;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span::Span;
use crate::span::shorthand::shorthand_span;
use crate::span::text::in_block::text_span_in_block;
use nom::Parser;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::multi::many1;
use nom::{IResult, branch::alt, bytes::complete::tag};

pub fn list_item_text<'a>(
    source: &'a str,
    _config: &'a Config,
    _parent: &'a BlockParent,
    parent_kind: &'a str,
) -> IResult<&'a str, Block> {
    let (source, _) = not((tag("-"), space1)).parse(source)?;
    Ok((
        source,
        Block::ListItemText {
            kind: format!("{}-text", parent_kind),
            spans: vec![],
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block::Block;
    use crate::block_metadata::parent::BlockParent;
    use crate::helpers::*;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::path::PathBuf;

    #[test]
    #[ignore]
    fn list_item_text_tests() {
        let config = Config::default();
        let file_list = get_file_list(
            &PathBuf::from("src/block/list_item_text/tests"),
            &vec!["txt".to_string()],
        )
        .unwrap();
        for source_path in file_list {
            if let Ok(data) = get_test_data(&source_path) {
                let result = list_item_text(
                    &data.0,
                    &config,
                    &BlockParent::ListItem,
                    "list-item",
                )
                .unwrap();
                let left_content = (
                    data.3.clone(),
                    serde_json::from_str::<Block>(&data.1).unwrap(),
                );
                let right_content = (data.3.clone(), result.1);
                assert_eq!(left_content, right_content);
                let left_content = (data.3.clone(), data.2.trim_end());
                let right_content = (data.3.clone(), result.0);
                assert_eq!(left_content, right_content);
            } else {
                assert!(false);
            }
        }
    }
}
