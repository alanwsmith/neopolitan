use crate::block::Block;
use crate::block::CsvData;
use crate::block_metadata::block_metadata;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span::Span;
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

// TODO: In docs, note that you can do custom
// delimeters.

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
            let mut delimeter = ",".to_string();
            metadata.attrs.iter().for_each(|a| match a.0.as_str() {
                "delimeter" => match &a.1[0] {
                    Span::Text { content , ..} => {
                        delimeter = content.to_string();

                    }
                    _ => (),
                },
                _ => (),
            });
            Ok((
                source,
                Block::Csv {
                    attrs: metadata.attrs,
                    data: parse_csv(body_base, &delimeter),
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

fn parse_csv(source: &str, delimeter: &String) -> CsvData {
    let mut items: Vec<Vec<String>> = vec![];
    let mut errors = vec![];
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .has_headers(false)
        .delimiter(delimeter.as_bytes()[0])
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
    use crate::span::Span;

    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn csv_block_test() {
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

    #[test]
    fn csv_change_delimeter() {
        let source = "-- csv\n-- delimeter: |\n\ng|h|i\nd|e|f\n\n";
        let config = Config::default();
        let parent = BlockParent::Page;
        let mut attrs = BTreeMap::new();
        attrs.insert(
            "delimeter".to_string(),
            vec![Span::Text {
                content: "|".to_string(),
kind: "text-span".to_string()
            }],
        );
        let left = Block::Csv {
            attrs,
            data: CsvData::Ok(vec![
                vec!["g".to_string(), "h".to_string(), "i".to_string()],
                vec!["d".to_string(), "e".to_string(), "f".to_string()],
            ]),
            end_block: None,
            flags: vec![],
            kind: "csv".to_string(),
        };
        let right = csv_block_full(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn csv_block_test_chomp_line_space() {
        let source = "    -- csv\n\nx, y, z\nd, e, f\n\n";
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Csv {
            attrs: BTreeMap::new(),
            data: CsvData::Ok(vec![
                vec!["x".to_string(), "y".to_string(), "z".to_string()],
                vec!["d".to_string(), "e".to_string(), "f".to_string()],
            ]),
            end_block: None,
            flags: vec![],
            kind: "csv".to_string(),
        };
        let right = csv_block_full(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn csv_block_error_test() {
        let source = "-- csv\n\na,b,c\nx";
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Csv {
            attrs: BTreeMap::new(),
            data: CsvData::Error("Could not parse CSV".to_string()),
            end_block: None,
            flags: vec![],
            kind: "csv".to_string(),
        };
        let right = csv_block_full(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn csv_block_none_test() {
        let source = "-- csv\n\n";
        let config = Config::default();
        let parent = BlockParent::Page;
        let left = Block::Csv {
            attrs: BTreeMap::new(),
            data: CsvData::None,
            end_block: None,
            flags: vec![],
            kind: "csv".to_string(),
        };
        let right = csv_block_full(source, &config, &parent).unwrap().1;
        assert_eq!(left, right);
    }

    //
}
