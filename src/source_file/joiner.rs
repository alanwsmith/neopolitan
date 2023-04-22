use crate::block::block::Block;
use crate::snippet::snippet::Snippet;
use crate::snippet::snippet::SnippetAttribute;
use html_escape;

pub fn joiner(children: &Option<Vec<Block>>) -> Vec<String> {
    let mut joined: Vec<String> = vec![];
    children.as_ref().unwrap().iter().for_each(|block| {
        let mut assembler: Vec<String> = vec![];
        match block {
            Block::Text { snippets } => {
                for snippet in snippets.as_ref().unwrap() {
                    match snippet {

                        Snippet::Abbr { string } => {
                            assembler.push(string.as_ref().unwrap().to_string());
                        }

                        Snippet::Plain { text } => {
                            let new_thing = text.as_ref().unwrap();
                            assembler.push(new_thing.to_string());
                            ()
                        }

                        Snippet::Kbd { text, attributes } => {
                            let mut content = String::from("<kbd");
                            match attributes {
                                Some(attrs) => {
                                    attrs.iter().for_each(|x| match x {
                                        SnippetAttribute::Attribute { key, value } => {
                                            content.push_str(" ");
                                            content.push_str(key.as_ref().unwrap().as_str());
                                            content.push_str(r#"=""#);
                                            content.push_str(value.as_ref().unwrap().as_str());
                                            content.push_str(r#"""#);
                                            ()
                                        }
                                    });
                                }
                                None => (),
                            }
                            content.push_str(">");
                            content.push_str(text.as_ref().unwrap());
                            content.push_str("</kbd>");
                            assembler.push(content.to_string());
                            ()
                        }

                        Snippet::Link {
                            attributes,
                            text,
                            url,
                        } => {
                            let mut content = String::from(r#"<a href=""#);
                            content.push_str(url.as_ref().unwrap().as_str());
                            content.push_str(r#"""#);
                            match attributes {
                                Some(attrs) => {
                                    attrs.iter().for_each(|x| match x {
                                        SnippetAttribute::Attribute { key, value } => {
                                            content.push_str(" ");
                                            content.push_str(key.as_ref().unwrap().as_str());
                                            content.push_str(r#"=""#);
                                            content.push_str(value.as_ref().unwrap().as_str());
                                            content.push_str(r#"""#);
                                            ()
                                        }
                                    });
                                }
                                None => (),
                            }
                            content.push_str(">");
                            content.push_str(
                                html_escape::encode_text(text.as_ref().unwrap().as_str()).as_ref(),
                            );
                            content.push_str("</a>");
                            assembler.push(content.to_string());
                            ()
                        }
                    }
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
    use crate::snippet::snippet::SnippetAttribute;
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

    #[test]
    pub fn kbd() {
        let source = Some(vec![Block::Text {
            snippets: Some(vec![Snippet::Kbd {
                attributes: None,
                text: Some("heavy black lines".to_string()),
            }]),
        }]);
        let expected: Vec<String> = vec![r#"<kbd>heavy black lines</kbd>"#.to_string()];
        let result = joiner(&source);
        assert_eq!(expected, result);
    }

    #[test]
    pub fn kbd_with_attributes() {
        let source = Some(vec![Block::Text {
            snippets: Some(vec![Snippet::Kbd {
                attributes: Some(vec![SnippetAttribute::Attribute {
                    key: Some("class".to_string()),
                    value: Some("foxtrot".to_string()),
                }]),
                text: Some("heavy black lines".to_string()),
            }]),
        }]);
        let expected: Vec<String> =
            vec![r#"<kbd class="foxtrot">heavy black lines</kbd>"#.to_string()];
        let result = joiner(&source);
        assert_eq!(expected, result);
    }
}
