use serde::Serialize;
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Snippet {
   Plain {text: Option<String>},
AbbreviationTag
{ string: Option<String> },
BringAttentionTag
{ string: Option<String> },
ButtonTag
{ string: Option<String> },
DataTag
{ string: Option<String> },
DeleteTag
{ string: Option<String> },
DefinitionTag
{ string: Option<String> },
EmphasisTag
{ string: Option<String> },
IdiomaticTextTag
{ string: Option<String> },
InsertTag  
{ string: Option<String> },
KeyboardInput
{ string: Option<String> },
LabelTag
{ string: Option<String> },
LegendTag
{ string: Option<String> },
MeterTag
{ string: Option<String> },
ObjectTag
{ string: Option<String> },
ProgressTag
{ string: Option<String> },
QuotationTag
{ string: Option<String> },
StrikethroughTag 
{ string: Option<String> },
SampleOutputTag
{ string: Option<String> },
SmallTextTag
{ string: Option<String> },
SpanTag
{ string: Option<String> },
StrongTag
{ string: Option<String> },
SubscriptTag
{ string: Option<String> },
SuperscriptTag
{ string: Option<String> },
TimeTag
{ string: Option<String> },
UnarticulatedAnnotationTag
{ string: Option<String> },
VariableTag
{ string: Option<String> },
}