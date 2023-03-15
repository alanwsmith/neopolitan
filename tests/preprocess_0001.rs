#[cfg(test)]
mod tests {

    use neopolitan::prep::prep;
    use neopolitan::rawblock::RawBlock;

    #[test]
    fn test_prep_0001() {
        let source = "-> TITLE\n\nNeopolitan Test";
        let expected: Vec<RawBlock> = vec![RawBlock::Title {
            text: "Neopolitan Test".to_string(),
        }];
        let result = prep(source);
        assert_eq!(expected, result.unwrap().1);
    }
}
