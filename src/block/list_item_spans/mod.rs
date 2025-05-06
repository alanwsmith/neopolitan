#![allow(unused)]
use crate::block::Block;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span::Span;
use crate::span::empty_line_or_lines_after_line_ending_or_eof::empty_line_or_lines_after_line_ending_or_eof;
use crate::span::escaped_character_in_block::escaped_character_in_block;
use crate::span::shorthand::shorthand_span;
use crate::span::single_character_allowed_in_block::single_character_allowed_in_block;
use crate::span::single_line_ending::single_line_ending;
use crate::span::text::in_block::text_span_in_block;
use crate::span::text_in_block::text_in_block;
use nom::Parser;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::combinator::not;
// use nom::multi::many0;
use nom::multi::many1;
use nom::{IResult, branch::alt, bytes::complete::tag};

// NOTE: This is currently using ``many1`` which means
// there has to be something after the ``-`` in the
// source file (i.e. you can't have an empty list
// item). I think that if that becomes necessary
// the least complicated approach would be to
// have a ``<<empty>>`` or something that
// allows the ``many1`` to work. (dealing with
// ``many0`` adds a lot of complication)

pub fn list_item_spans<'a>(
    source: &'a str,
    _config: &'a Config,
    _parent: &'a BlockParent,
    parent_kind: &'a str,
) -> IResult<&'a str, Block> {
    let (source, spans) = many1(alt((
        text_in_block,
        single_line_ending,
        escaped_character_in_block,
        single_character_allowed_in_block,
    )))
    .parse(source)?;
    let (source, _) =
        empty_line_or_lines_after_line_ending_or_eof.parse(source)?;
    Ok((
        source,
        Block::ListItemSpans {
            kind: format!("{}-spans", parent_kind),
            spans,
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
        let source_dir = PathBuf::from("src/block/list_item_spans/tests");
        let config = Config::default();
        let parent = BlockParent::ListItem;
        let parent_kind = "list-item";
        let test_file_list =
            get_file_list(&source_dir, &vec!["neotest".to_string()]).unwrap();
        for source_path in test_file_list {
            println!("test {}", &source_path.display());
            match run_block_test_case_with_source_config_parent_parent_kind(
                &source_path,
                &config,
                &parent,
                &parent_kind,
                &list_item_spans,
            ) {
                TestBlockPayload::ExpectedError => {
                    println!("got expected error");
                    assert!(true);
                }
                TestBlockPayload::Ok {
                    left_content,
                    right_content,
                    left_remainder,
                    right_remainder,
                } => {
                    assert_eq!(left_content, right_content);
                    assert_eq!(left_remainder, right_remainder);
                }
                TestBlockPayload::ShouldHaveErroredButDidNot => {
                    dbg!("########### THIS SHOULD HAVE ERRORED BUT DID NOTE");
                    assert!(false);
                }
                TestBlockPayload::UnexpectedError => {
                    dbg!("########### GOT AN ERROR THAT WAS NOT EXPECTED");
                    assert!(false);
                }
                TestBlockPayload::ExpectedError => {
                    assert!(true);
                }
                TestBlockPayload::Skip => {
                    dbg!("########### SKIPPING ############");
                    assert!(true);
                }
            }
        }
    }
}
