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
                        Snippet::Kbd { attributes, text } => {
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

    // dbg!(children);
    // "asdf".to_string()
    joined
}
