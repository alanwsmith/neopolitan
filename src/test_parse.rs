#[cfg(test)]
mod tests {
    use crate::parse::parse;
    use crate::parse::parse_dev;
    use crate::parse::Page;
    // juse serde::{Deserialize, Serialize};
    // use serde_json::Result;
    use std::fs;

    // #[test]
    // fn test_001() {
    //     let source = fs::read_to_string("./test_targets/001.basic.neo").unwrap();
    //     let expected: Page = serde_json::from_str(
    //         fs::read_to_string("./test_targets/001.basic.json")
    //             .unwrap()
    //             .as_str(),
    //     )
    //     .unwrap();
    //     let result = parse(&source);
    //     // dbg!(&expected);
    //     // dbg!(&result);
    //     assert_eq!(expected, result);
    // }

    #[test]
    fn test_002() {
        let source = fs::read_to_string("./test_targets/002.basic.neo").unwrap();
        let expected: Page = serde_json::from_str(
            fs::read_to_string("./test_targets/002.basic.json")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let result = parse_dev(&source);
        dbg!(&expected);
        dbg!(&result);
        assert_eq!(expected, result);
    }
}
