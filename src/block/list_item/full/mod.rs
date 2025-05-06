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

// TODO: Make sure to pass in the top level kind
// so that this can use it for the output kind
// to match? Or should it always just be a list item?
// Probably makes sense to send the kind as an
// option with the fallback to the category
// (just like everything else)

pub fn list_item_block_full<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
    parent_kind: &'a str,
) -> IResult<&'a str, Block> {
    let (source, _) = (space0, tag("-"), space1).parse(source)?;
    let (source, children) =
        many0(|src| text_block(src, config, &BlockParent::Basic))
            .parse(source)?;
    Ok((
        source,
        Block::ListItem {
            children,
            kind: format!("{}-item", parent_kind),
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

    // #[test]
    // #[ignore]
    // fn list_item_block_full_tests() {
    //     let config = Config::default();
    //     let file_list = get_file_list(
    //         &PathBuf::from("src/block/list_item/full/tests"),
    //         &vec!["txt".to_string()],
    //     )
    //     .unwrap();
    //     for source_path in file_list {
    //         if let Ok(data) = get_test_data(&source_path) {
    //             let result = list_item_block_full(
    //                 &data.0,
    //                 &config,
    //                 &BlockParent::List,
    //                 "list",
    //             )
    //             .unwrap();
    //             let left_content = (
    //                 data.3.clone(),
    //                 serde_json::from_str::<Block>(&data.1).unwrap(),
    //             );
    //             let right_content = (data.3.clone(), result.1);
    //             assert_eq!(left_content, right_content);
    //             let left_content = (data.3.clone(), data.2.trim_end());
    //             let right_content = (data.3.clone(), result.0);
    //             assert_eq!(left_content, right_content);
    //         } else {
    //             assert!(false);
    //         }
    //     }
    // }

    //
}
