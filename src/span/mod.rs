pub mod code;
pub mod empty_line_or_lines_after_line_ending_or_eof;
pub mod escaped;
pub mod escaped_character_in_block;
pub mod shorthand;
pub mod single_character_allowed_in_block;
pub mod single_line_ending;
pub mod strikethrough_shorthand;
pub mod tag_shorthand;
pub mod text;
pub mod text_in_block;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "category", rename_all = "lowercase")]
pub enum Span {
    //
    // TODO: Move these so the values are directly
    // in the enum and not nested structs
    //
    #[serde(rename = "code-shorthand")]
    Code {
        attrs: BTreeMap<String, Vec<Span>>,
        flags: Vec<String>,
        kind: String,
        spans: Vec<Span>,
    },
    #[serde(rename = "empty-line-or-lines")]
    EmptyLineOrLines {
        // TODO: Remove kind from here since
        // spaces don't have them.
        kind: String,
    },
    // #[serde(rename = "emphasis-span")]
    // EmphasisShorthand(EmphasisShorthandV42),
    //
    // #[serde(rename = "escaped-span")]
    // EscapedCharacter(EscapedCharacterSpanV42),
    #[serde(rename = "escaped-character")]
    Escaped { content: String, kind: String },
    //
    // #[serde(rename = "footnote-span")]
    // FootnoteShorthand(FootnoteShorthandV42),
    // #[serde(rename = "html-span")]
    // HtmlShorthand(HtmlShorthandV42),
    // #[serde(rename = "image-span")]
    // ImageShorthand(ImageShorthandV42),
    // #[serde(rename = "link-span")]
    // LinkShorthand(LinkShorthandV42),
    // #[serde(rename = "mark-span")]
    // MarkShorthand(MarkShorthandV42),
    //
    // // TODO: rename to TagSpan
    // #[serde(rename = "tag")]
    // NamedSpan(NamedSpanV42),
    //
    // #[serde(rename = "text")]
    // TextSpan {
    //     kind: String,
    //     text: String,
    // },
    #[serde(rename = "space")]
    Space { kind: String },
    #[serde(rename = "strikethrough-shorthand")]
    StrikethroughShorthand {
        attrs: BTreeMap<String, Vec<Span>>,
        flags: Vec<String>,
        kind: String,
        spans: Vec<Span>,
    },
    #[serde(rename = "text")]
    Text { content: String, kind: String },
    // #[serde(rename = "strong-span")]
    // StrongShorthand(StrongShorthandV42),
    // #[serde(rename = "strike-span")]
    // StrikethroughShorthand(StrikethroughShorthandV42),
    // #[serde(rename = "underline-span")]
    // UnderlineShorthand(UnderlineShorthandV42),
}
