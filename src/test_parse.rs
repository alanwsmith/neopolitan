#[cfg(test)]
mod tests {
    use crate::parse::parse;
    use crate::parse::parse_dev;
    use crate::parse::Page;
    use std::fs;

    // #[test]
    // fn test_002() {
    //     let source = fs::read_to_string("./test_targets/002.basic.neo").unwrap();
    //     let expected: Page = serde_json::from_str(
    //         fs::read_to_string("./test_targets/002.basic.json")
    //             .unwrap()
    //             .as_str(),
    //     )
    //     .unwrap();
    //     let result = parse(&source);
    //     // dbg!(&result);
    //     assert_eq!(expected, result);
    // }

    // #[test]
    // fn test_003() {
    //     let source = fs::read_to_string("./test_targets/003.basic.neo").unwrap();
    //     let expected: Page = serde_json::from_str(
    //         fs::read_to_string("./test_targets/003.basic.json")
    //             .unwrap()
    //             .as_str(),
    //     )
    //     .unwrap();
    //     let result = parse(&source);
    //     // dbg!(&result);
    //     assert_eq!(expected, result);
    // }

    #[test]
    fn test_004() {
        let source = fs::read_to_string("./test_targets/004.basic.neo").unwrap();
        let expected: Page = serde_json::from_str(
            fs::read_to_string("./test_targets/004.basic.json")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let result = parse_dev(&source);
        dbg!(&result);
        assert_eq!(expected, result);
    }
}
