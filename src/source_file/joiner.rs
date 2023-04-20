use crate::block::block::Block;
use crate::snippet::snippet::Snippet;

pub fn joiner(children: &Option<Vec<Block>>) -> Vec<String> {
    let mut joined: Vec<String> = vec![];
    children.as_ref().unwrap().iter().for_each(|block| {
        let mut assembler: Vec<String> = vec![];
        match block {
            Block::Text { snippets } => {
                for snippet in snippets.as_ref().unwrap() {
                    match snippet {
                        Snippet::Plain { text } => {
                            let new_thing = text.as_ref().unwrap();
                            assembler.push(new_thing.to_string());
                            ()
                        }
                        Snippet::Kbd { text, .. } => {
                            let mut content = String::from("<kbd");
                            content.push_str(">");
                            content.push_str(text.as_ref().unwrap());
                            content.push_str("</kbd>");
                            assembler.push(content.to_string());
                            ()
                        }
                        _ => (),
                    }
                    // dbg!(snippet);
                }
                ()
            }
            _ => (),
        }
        joined.push(assembler.join("\n"));
    });
    joined
}

#[cfg(test)]
mod test {
    use crate::block::block::Block;
    use crate::snippet::snippet::Snippet;
    use crate::source_file::joiner::joiner;

    #[test]
    pub fn basic_joiner() {
        let source = Some(vec![Block::Text {
            snippets: Some(vec![Snippet::Plain {
                text: Some("asdf".to_string()),
            }]),
        }]);
        let expected: Vec<String> = vec![r#"asdf"#.to_string()];
        let result = joiner(&source);
        assert_eq!(expected, result);
    }
}
