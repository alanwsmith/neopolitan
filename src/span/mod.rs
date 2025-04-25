pub mod plain_text;

use crate::span::plain_text::PlainTextSpan;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "category", rename_all = "lowercase")]
pub enum Span {
    // #[serde(rename = "code-shorthand")]
    // CodeShorthand(CodeShorthandV42),
    // #[serde(rename = "emphasis-shorthand")]
    // EmphasisShorthand(EmphasisShorthandV42),
    // #[serde(rename = "escaped-character")]
    // EscapedCharacter(EscapedCharacterSpanV42),
    // #[serde(rename = "footnote-shorthand")]
    // FootnoteShorthand(FootnoteShorthandV42),
    // #[serde(rename = "html-shorthand")]
    // HtmlShorthand(HtmlShorthandV42),
    // #[serde(rename = "image-shorthand")]
    // ImageShorthand(ImageShorthandV42),
    // #[serde(rename = "link-shorthand")]
    // LinkShorthand(LinkShorthandV42),
    // #[serde(rename = "mark-shorthand")]
    // MarkShorthand(MarkShorthandV42),
    // #[serde(rename = "named-span")]
    // NamedSpan(NamedSpanV42),
    #[serde(rename = "plain-text")]
    PlainText(PlainTextSpan),
    // #[serde(rename = "strong-shorthand")]
    // StrongShorthand(StrongShorthandV42),
    // #[serde(rename = "strike-shorthand")]
    // StrikethroughShorthand(StrikethroughShorthandV42),
    // #[serde(rename = "underline-shorthand")]
    // UnderlineShorthand(UnderlineShorthandV42),
}
