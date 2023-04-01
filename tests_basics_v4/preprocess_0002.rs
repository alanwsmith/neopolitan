#[cfg(test)]
mod tests {

    // This tests a bunch of headers as a basic
    // smoketest.

    use neopolitan::prep::prep;
    use neopolitan::rawblock::RawBlock;

    #[test]
    fn test_prep_0001() {
        let source = r#"-> TITLE

Neopolitan Test

-> BLURB

This is the blurb 

-> P

Here's a paragraph

And another one

-> CODE
>> rust

This shows an attribute. 

-> ATTRIBUTES
>> type: post 
>> status: unpublished

-> DIV

A div

-> H2

Okay, that's probably enough for now

"#;
        let expected: Vec<RawBlock> = vec![
            RawBlock::Title {
                text: "Neopolitan Test".to_string(),
            },
            RawBlock::Blurb {
                text: "This is the blurb".to_string(),
            },
            RawBlock::P {
                text: "Here's a paragraph\n\nAnd another one".to_string(),
            },
            RawBlock::Code {
                text: ">> rust\n\nThis shows an attribute.".to_string(),
            },
            RawBlock::Attributes {
                text: ">> type: post \n>> status: unpublished".to_string(),
            },
            RawBlock::Div {
                text: "A div".to_string(),
            },
            RawBlock::H2 {
                text: "Okay, that's probably enough for now".to_string(),
            },
        ];
        let result = prep(source);
        assert_eq!(expected, result.unwrap().1);
    }
}
