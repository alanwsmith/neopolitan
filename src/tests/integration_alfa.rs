#[test]
fn test1() {
    let source = "-> title\n\nHere it is";
    let expected = Wrapper::Page {
        children: Some(vec![Section::Title {
            attributes: None,
            children: Some(vec![Block::P {
                attributes: None,
                children: Some(vec![Content::Text {
                    value: Some("Here it is".to_string()),
                }]),
            }]),
        }]),
    };
    let (_, result) = parse(source).unwrap();
    assert_eq!(expected, result);
}
