use crate::block::block::Block;
use crate::snippet::snippet::Snippet;

pub fn joiner(children: &Option<Vec<Block>>) -> Vec<String> {
    let mut joined: Vec<String> = vec![];
    children.as_ref().unwrap().iter().for_each(|block| {
        let mut assembler = String::from("");
        match block {
            Block::Text { snippets } => {
                for snippet in snippets.as_ref().unwrap() {
                    match snippet {
                        Snippet::Plain { text } => {
                            assembler.push_str(text.as_ref().unwrap());
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
        joined.push(assembler);
    });

    // dbg!(children);
    // "asdf".to_string()
    joined
}

