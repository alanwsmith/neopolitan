use serde::Serialize;
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Snippet {
   Plain {text: Option<String>},
AbbreviationTag
{ text: Option<String> },
BringAttentionTag
{ text: Option<String> },
ButtonTag
{ text: Option<String> },
DataTag
{ text: Option<String> },
DeleteTag
{ text: Option<String> },
DefinitionTag
{ text: Option<String> },
EmphasisTag
{ text: Option<String> },
IdiomaticTextTag
{ text: Option<String> },
InsertTag  
{ text: Option<String> },
KeyboardInput
{ text: Option<String> },
LabelTag
{ text: Option<String> },
LegendTag
{ text: Option<String> },
MeterTag
{ text: Option<String> },
ObjectTag
{ text: Option<String> },
ProgressTag
{ text: Option<String> },
QuotationTag
{ text: Option<String> },
StrikethroughTag 
{ text: Option<String> },
SampleOutputTag
{ text: Option<String> },
SmallTextTag
{ text: Option<String> },
SpanTag
{ text: Option<String> },
StrongTag
{ text: Option<String> },
SubscriptTag
{ text: Option<String> },
SuperscriptTag
{ text: Option<String> },
TimeTag
{ text: Option<String> },
UnarticulatedAnnotationTag
{ text: Option<String> },
VariableTag
{ text: Option<String> },
}