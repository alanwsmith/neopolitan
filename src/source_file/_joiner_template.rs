use crate::block::block::Block;
use crate::snippet::snippet_enum::Snippet;

pub fn joiner(children: &Option<Vec<Block>>) -> Vec<String> {
    let mut joined: Vec<String> = vec![];
    children.as_ref().unwrap().iter().for_each(|block| {
        let mut assembler: Vec<String> = vec![];
        match block {
            Block::Text { snippets } => {
                for snippet in snippets.as_ref().unwrap() {
                    match snippet {
$ENUMS

                        Snippet::Plain { text } => {
                            let new_thing = text.as_ref().unwrap();
                            assembler.push(new_thing.to_string());
                            ()
                        } 
                    }
                }
            }
            _ => (),
        }
        joined.push(assembler.join(" "));
    });
    joined
}
