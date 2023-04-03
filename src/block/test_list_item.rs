use crate::block::block::Block;
use crate::block::list_item::list_item;
use crate::content::content::Content;

#[test]
fn basic() {
    let source = "- alfa bravo\n\n";
    let expected = Ok((
        "",
        Block::ListItem {
            attributes: None,
            children: Some(vec![Block::P {
                attributes: None,
                children: Some(vec![Content::Text {
                    text: Some("alfa bravo".to_string()),
                }]),
            }]),
        },
    ));
    let result = list_item(source);
    assert_eq!(expected, result);
}

// #[test]
// fn no_lines_at_end() {
//     let source = "charlie delta";
//     let expected = Ok((
//         "",
//         Block::P {
//             attributes: None,
//             children: Some(vec![Content::Text {
//                 text: Some("charlie delta".to_string()),
//             }]),
//         },
//     ));
//     let result = p(source);
//     assert_eq!(expected, result);
// }

// #[test]
// fn multiple_lines() {
//     let lines = ["Split the log with a quick,", "sharp blow."].join("\n");
//     let source = lines.as_str();
//     let expected = Ok((
//         "",
//         Block::P {
//             attributes: None,
//             children: Some(vec![Content::Text {
//                 text: Some("Split the log with a quick,\nsharp blow.".to_string()),
//             }]),
//         },
//     ));
//     let result = p(source);
//     assert_eq!(expected, result);
// }
