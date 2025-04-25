use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "category", rename_all = "lowercase")]
pub enum Span {
    //
    // TODO: Move these so the values are directly
    // in the enum and not nested structs
    //

    // #[serde(rename = "code-span")]
    // CodeShorthand(CodeShorthandV42),
    // #[serde(rename = "emphasis-span")]
    // EmphasisShorthand(EmphasisShorthandV42),
    // #[serde(rename = "escaped-span")]
    // EscapedCharacter(EscapedCharacterSpanV42),
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
    // #[serde(rename = "named-span")]
    // NamedSpan(NamedSpanV42),
    #[serde(rename = "text-span")]
    TextSpan { kind: String, text: String },
    // #[serde(rename = "strong-span")]
    // StrongShorthand(StrongShorthandV42),
    // #[serde(rename = "strike-span")]
    // StrikethroughShorthand(StrikethroughShorthandV42),
    // #[serde(rename = "underline-span")]
    // UnderlineShorthand(UnderlineShorthandV42),
}
