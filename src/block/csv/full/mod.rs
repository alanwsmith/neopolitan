use crate::block::Block;
use crate::block::CsvData;
use crate::block_metadata::block_metadata;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span_metadata::strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use csv::ReaderBuilder;
use csv::Trim;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::rest;
use nom::sequence::terminated;
use nom::{IResult, bytes::complete::tag};

pub fn csv_block_full<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a BlockParent,
) -> IResult<&'a str, Block> {
    let (source, _) = (space0, tag("--"), space1).parse(source)?;
    let (source, kind) =
        terminated(is_not("/ \t\r\n"), space0_line_ending_or_eof)
            .parse(source)?;
    if config.block_category_kinds.csv.contains(&kind.to_string()) {
        let (source, metadata) = block_metadata(source, config, parent)?;
        let (source, _) = multispace0.parse(source)?;
        let (source, body_base) =
            alt((take_until("-- "), rest)).parse(source)?;
        if body_base.trim() == "" {
            Ok((
                source,
                Block::Csv {
                    attrs: metadata.attrs,
                    data: CsvData::None,
                    end_block: None,
                    flags: metadata.flags,
                    kind: kind.to_string(),
                },
            ))
        } else {
            Ok((
                source,
                Block::Csv {
                    attrs: metadata.attrs,
                    data: parse_csv(body_base),
                    end_block: None,
                    flags: metadata.flags,
                    kind: kind.to_string(),
                },
            ))
        }
    } else {
        Err(nom::Err::Error(nom::error::Error {
            input: source,
            code: nom::error::ErrorKind::Tag,
        }))
    }
}

fn parse_csv(source: &str) -> CsvData {
    let mut items: Vec<Vec<String>> = vec![];
    let mut errors = vec![];
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .has_headers(false)
        .delimiter(b',')
        .from_reader(source.as_bytes());
    for result in rdr.deserialize() {
        match result {
            Ok(record) => items.push(record),
            Err(e) => errors.push(e.to_string()),
        }
    }
    if errors.len() == 0 {
        CsvData::Ok(items)
    } else {
        CsvData::Error("Could not parse CSV".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn solo_csv_block_test() {
        let source = "-- csv\n\na, b, c\nd, e, f\n\n";
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Csv {
            attrs: BTreeMap::new(),
            data: CsvData::Ok(vec![
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                vec!["d".to_string(), "e".to_string(), "f".to_string()],
            ]),
            end_block: None,
            flags: vec![],
            kind: "csv".to_string(),
        };
        let right = csv_block_full(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

    // #[test]
    // fn csv_block_test_chomp_leading_line_space() {
    //     let source = "    -- json\n\n{ \"tango\": \"foxtrot\" }";
    //     let config = Config::default();
    //     let parent = BlockParent::Page;
    //     let left = Block::Csv {
    //         attrs: BTreeMap::new(),
    //         data: CsvData::Ok(
    //             serde_json::from_str(r#"{"tango": "foxtrot"}"#).unwrap(),
    //         ),
    //         end_block: None,
    //         flags: vec![],
    //         kind: "json".to_string(),
    //     };
    //     let right = csv_block_full(source, &config, &parent).unwrap().1;
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn csv_block_error_test() {
    //     let source = "-- json\n\nxxx";
    //     let config = Config::default();
    //     let parent = BlockParent::Page;
    //     let left = Block::Csv {
    //         attrs: BTreeMap::new(),
    //         data: CsvData::Error(
    //             "expected value at line 1 column 1".to_string(),
    //         ),
    //         end_block: None,
    //         flags: vec![],
    //         kind: "json".to_string(),
    //     };
    //     let right = csv_block_full(source, &config, &parent).unwrap().1;
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn csv_block_none_test() {
    //     let source = "-- json\n\n";
    //     let config = Config::default();
    //     let parent = BlockParent::Page;
    //     let left = Block::Csv {
    //         attrs: BTreeMap::new(),
    //         data: CsvData::None,
    //         end_block: None,
    //         flags: vec![],
    //         kind: "json".to_string(),
    //     };
    //     let right = csv_block_full(source, &config, &parent).unwrap().1;
    //     assert_eq!(left, right);
    // }

    //
}
