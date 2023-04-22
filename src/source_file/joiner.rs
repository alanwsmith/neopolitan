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
Snippet::AbbreviationTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::BringAttentionTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::ButtonTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::DataTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::DeleteTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::DefinitionTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::EmphasisTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::IdiomaticTextTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::InsertTag  
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::KeyboardInput
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::LabelTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::LegendTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::MeterTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::ObjectTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::ProgressTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::QuotationTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::StrikethroughTag 
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::SampleOutputTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::SmallTextTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::SpanTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::StrongTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::SubscriptTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::SuperscriptTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::TimeTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::UnarticulatedAnnotationTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::VariableTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }
Snippet::LinkTag
    { string } => {
   assembler.push(string.as_ref().unwrap().to_string()); }

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
