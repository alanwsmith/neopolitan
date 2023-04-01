#![allow(unused_imports)]
#[cfg(test)]
mod tests {

    use neopolitan::process::process;

    #[test]
    fn test_output_0001() {
        let input = "-> TITLE\n\nNeopolitan Test";
        let expected = r#"<h1 class="title">Neopolitan Test</h1>"#;
        let result = process(input);
        assert_eq!(expected, result);
    }
}
