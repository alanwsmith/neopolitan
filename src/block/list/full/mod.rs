#![allow(unused)]
use crate::block::Block;
use crate::block::text_block::text_block;
use crate::block_metadata::block_metadata;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span_metadata::strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::{IResult, bytes::complete::tag};

// TODO: Make sure there has to be an empty line below
// the block, flag, and atter tokens before the content
// otherwise it should throw a parsing error if it's
// after the block token or a flag, and it gets
// slurped into the attribute if it's an attr
//
// TODO: Check the blank line thing in all block types

pub fn list_block_full<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
) -> IResult<&'a str, Block> {
    let (source, _) = (space0, tag("--"), space1).parse(source)?;
    let (source, kind) =
        terminated(is_not("/ \t\r\n"), space0_line_ending_or_eof)
            .parse(source)?;
    let (source, metadata) = block_metadata(source, config, parent)?;
    let (source, _) = multispace0.parse(source)?;
    let (source, children) =
        many0(|src| text_block(src, config, &BlockParent::Basic))
            .parse(source)?;
    Ok((
        source,
        Block::List {
            attrs: metadata.attrs,
            children,
            end_block: None,
            flags: metadata.flags,
            kind: kind.to_string(),
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
    fn solo_list_item_spans_tests() {
        let source_dir = PathBuf::from("src/block/list/full/tests");
        let config = Config::default();
        let parent = BlockParent::ListItem;
        let parent_kind = "list";
        let test_file_list =
            get_file_list(&source_dir, &vec!["neotest".to_string()]).unwrap();
        for source_path in test_file_list {
            println!("test {}", &source_path.display());
            match run_block_test_case_with_source_config_parent(
                &source_path,
                &config,
                &parent,
                &list_block_full,
            ) {
                TestBlockPayload::Ok {
                    left_content,
                    right_content,
                    left_remainder,
                    right_remainder,
                } => {
                    assert_eq!(left_content, right_content);
                    assert_eq!(left_remainder, right_remainder);
                }
                _ => {
                    assert!(false);
                }
            }
        }
    }

    //
}
