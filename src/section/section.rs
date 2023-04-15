use crate::block::block::Block;
use crate::section::aside::*;
use crate::section::attributes::*;
use crate::section::attributes_for_section::*;
use crate::section::blockquote::*;
use crate::section::blurb::*;
use crate::section::code_section::*;
use crate::section::code_start_end::*;
use crate::section::comment::*;
use crate::section::css::*;
use crate::section::div::*;
use crate::section::footnote::*;
use crate::section::h1::*;
use crate::section::h2::*;
use crate::section::h3::*;
use crate::section::h4::*;
use crate::section::h5::*;
use crate::section::h6::*;
use crate::section::html::*;
use crate::section::image::*;
use crate::section::list::*;
use crate::section::neo_example_start_end::*;
use crate::section::note::*;
use crate::section::notes::*;
use crate::section::olist::*;
use crate::section::p::*;
use crate::section::pre::*;
use crate::section::reference::*;
use crate::section::results::*;
use crate::section::script::*;
use crate::section::subtitle::*;
use crate::section::textarea::*;
use crate::section::tiktok::*;
use crate::section::title::*;
use crate::section::todo::*;
use crate::section::vimeo::*;
use crate::section::warning::*;
use crate::section::wc::*;
use crate::section::widget::*;
use crate::section::youtube::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Section {
    // This placeholder is for dev to stub out
    // things that are in progress
    Placeholder,
    // These are the actual section types
    AsideSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    AttributesSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    BlockquoteSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    BlurbSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    CodeSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    CodeStartEndSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    CommentSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    CSSSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    DivSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    FootnoteSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    H1Section {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    H2Section {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    H3Section {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    H4Section {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    H5Section {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    H6Section {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    HTMLSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    ImageSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    List {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    NeoExampleStartEndSection {
        attributes: Option<Vec<SectionAttribute>>,
        raw: Option<Block>,
        html: Option<String>,
    },
    NoteSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    NotesSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    OrderedListSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    Paragraphs {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    PreSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    ReferenceSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    ResultsSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    ScriptSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    Subtitle {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    TextareaSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    TikTokSection {
        attributes: Option<Vec<SectionAttribute>>,
    },
    Title {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    ToDoSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    VimeoSection {
        attributes: Option<Vec<SectionAttribute>>,
    },
    WarningSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    WebComponentSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    WidgetSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    YouTubeSection {
        attributes: Option<Vec<SectionAttribute>>,
    },
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (remainder, sec) = alt((
        alt((
            // note that this one is different from all the rest
            // since it has an explict stopping tag that's
            // not the start of the next section
            tuple((
                tag("-> startcode\n"),
                take_until("\n\n-> endcode"),
                tag("\n\n-> endcode"),
            ))
            .map(|t| code_start_end(t.1).unwrap().1),
            tuple((tag("-> aside\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| aside(t.1).unwrap().1),
            tuple((tag("-> attributes\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| attributes(t.1).unwrap().1),
            tuple((tag("-> blockquote\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| blockquote(t.1).unwrap().1),
            tuple((tag("-> code\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| code_section(t.1).unwrap().1),
            tuple((tag("-> comment\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| comment(t.1).unwrap().1),
            tuple((tag("-> div\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| div(t.1).unwrap().1),
            tuple((tag("-> note\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| note(t.1).unwrap().1),
            tuple((tag("-> title\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| title(t.1).unwrap().1),
            tuple((tag("-> p\n"), alt((take_until("\n\n-> "), rest)))).map(|t| p(t.1).unwrap().1),
            tuple((tag("-> subtitle\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| subtitle(t.1).unwrap().1),
            tuple((tag("-> list\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| list(t.1).unwrap().1),
            tuple((tag("-> vimeo\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| vimeo(t.1).unwrap().1),
            tuple((tag("-> warning\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| warning(t.1).unwrap().1),
            tuple((tag("-> youtube\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| youtube(t.1).unwrap().1),
        )),
        alt((
            tuple((tag("-> blurb\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| blurb(t.1).unwrap().1),
            tuple((tag("-> css\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| css(t.1).unwrap().1),
            tuple((tag("-> footnote\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| footnote(t.1).unwrap().1),
            tuple((tag("-> h1\n"), alt((take_until("\n\n-> "), rest)))).map(|t| h1(t.1).unwrap().1),
            tuple((tag("-> h2\n"), alt((take_until("\n\n-> "), rest)))).map(|t| h2(t.1).unwrap().1),
            tuple((tag("-> h3\n"), alt((take_until("\n\n-> "), rest)))).map(|t| h3(t.1).unwrap().1),
            tuple((tag("-> h4\n"), alt((take_until("\n\n-> "), rest)))).map(|t| h4(t.1).unwrap().1),
            tuple((tag("-> h5\n"), alt((take_until("\n\n-> "), rest)))).map(|t| h5(t.1).unwrap().1),
            tuple((tag("-> h6\n"), alt((take_until("\n\n-> "), rest)))).map(|t| h6(t.1).unwrap().1),
            tuple((tag("-> html\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| html(t.1).unwrap().1),
            tuple((tag("-> image\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| image(t.1).unwrap().1),
            tuple((tag("-> notes\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| notes(t.1).unwrap().1),
            tuple((tag("-> olist\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| olist(t.1).unwrap().1),
            tuple((tag("-> pre\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| pre(t.1).unwrap().1),
            tuple((tag("-> reference\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| reference(t.1).unwrap().1),
            tuple((tag("-> script\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| script(t.1).unwrap().1),
            tuple((tag("-> textarea\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| textarea(t.1).unwrap().1),
            tuple((tag("-> tiktok\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| tiktok(t.1).unwrap().1),
            tuple((tag("-> wc\n"), alt((take_until("\n\n-> "), rest)))).map(|t| wc(t.1).unwrap().1),
            tuple((tag("-> widget\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| widget(t.1).unwrap().1),
        )),
        alt((
            tuple((tag("-> todo\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| todo(t.1).unwrap().1),
            tuple((tag("-> todos\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| todo(t.1).unwrap().1),
            tuple((tag("-> results\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| results(t.1).unwrap().1),
            tuple((
                tag("-> startneoexample\n"),
                take_until("\n\n-> endneoexample"),
                tag("\n\n-> endneoexample"),
            ))
            .map(|t| neo_example_start_end(t.1).unwrap().1),
        )),
    ))(source)?;
    Ok((remainder, sec))
}
