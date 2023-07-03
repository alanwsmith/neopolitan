use crate::block::block::*;
use crate::section::aside::*;
use crate::section::attributes::*;
use crate::section::blockquote::*;
use crate::section::blurb::*;
use crate::section::canvas::*;
use crate::section::categories::*;
use crate::section::checklist::*;
use crate::section::code::*;
use crate::section::comment::*;
use crate::section::css::*;
use crate::section::details::*;
use crate::section::dlist::*;
use crate::section::ext::*;
use crate::section::footnote::*;
use crate::section::h1::*;
use crate::section::h2::*;
use crate::section::h3::*;
use crate::section::h4::*;
use crate::section::h5::*;
use crate::section::h6::*;
use crate::section::head::*;
use crate::section::hr::*;
use crate::section::html::*;
use crate::section::image::*;
use crate::section::include::*;
use crate::section::list::*;
use crate::section::list_enum::ListItem;
use crate::section::menu::*;
use crate::section::nav::*;
use crate::section::neoexample_start_end::*;
use crate::section::note::*;
use crate::section::notes::*;
use crate::section::object::*;
use crate::section::olist::*;
use crate::section::p::*;
use crate::section::pre::*;
use crate::section::reference::*;
use crate::section::results::*;
use crate::section::script::*;
use crate::section::section_attributes::SectionAttribute;
use crate::section::startdiv::*;
use crate::section::subtitle::*;
use crate::section::table::*;
use crate::section::textarea::*;
use crate::section::title::*;
use crate::section::todo::*;
use crate::section::todos::*;
use crate::section::vimeo::*;
use crate::section::warning::*;
use crate::section::widget::*;
use crate::section::youtube::*;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Section {
    NeoExampleStartEndSection {
        attributes: Option<Vec<SectionAttribute>>,
        html: Option<String>,
        raw: Option<String>,
    },
    AsideSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    AttributesSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    AudioSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    BlockquoteSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    BlurbSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    CanvasSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    CategoriesSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    ChecklistSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<ChecklistItem>>,
    },
    CodeSection {
        attributes: Option<Vec<SectionAttribute>>,
        attributes_string: Option<String>,
        title: Option<String>,
        language: Option<String>,
        raw: Option<String>,
    },
    CodeStartEndSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    CommentSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    CSSSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    DetailsSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    DescriptionListSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<DescriptionListItem>>,
    },
    ExternalSection {
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
    HeadSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    HRSection {
        attributes: Option<Vec<SectionAttribute>>,
    },
    HTMLSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    ImageSection {
        alt_text: Option<String>,
        attributes: Option<Vec<SectionAttribute>>,
        src: Option<String>,
    },
    IncludeSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    ListSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<ListItem>>,
    },
    MenuSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<ListItem>>,
    },
    NavSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<ListItem>>,
    },
    NoteSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
        title: Option<String>,
    },
    NotesSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<ListItem>>,
    },
    ObjectSection {
        attributes: Option<Vec<SectionAttribute>>,
    },
    OrderedListSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<ListItem>>,
    },
    ParagraphsSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    PreSection {
        attributes: Option<Vec<SectionAttribute>>,
        raw: Option<String>,
    },
    ReferenceSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    ResultsSection {
        attributes: Option<Vec<SectionAttribute>>,
        raw: Option<String>,
    },
    ScriptSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    StartDivSection {
        attributes: Option<Vec<SectionAttribute>>,
        html: Option<String>,
    },
    SubtitleSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    TableSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    TextareaSection {
        attributes: Option<Vec<SectionAttribute>>,
        raw: Option<String>,
    },
    TitleSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    TodoSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    TodosSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<TodosItem>>,
    },
    VimeoSection {
        attributes: Option<Vec<SectionAttribute>>,
        id: Option<String>,
    },
    WarningSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    WidgetSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    YouTubeSection {
        attributes: Option<Vec<SectionAttribute>>,
        id: Option<String>,
    },
    Placeholder,
}

// NOTE: Order matters here, the longer names
// need to go first so they shorter names don't
// yank the names out from under them

pub fn section(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = multispace0(source)?;

    let (remainder, section) = alt((
        alt((
            tuple((
                tag_no_case("-> startneoexample"),
                not_line_ending,
                line_ending,
                take_until("\n\n-> endneoexample"),
                tag_no_case("\n\n-> endneoexample"),
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| neoexample_start_end(t.3).unwrap().1),
            tuple((
                tag_no_case("-> aside"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| aside(t.3).unwrap().1),
            tuple((
                tag_no_case("-> blockquote"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| blockquote(t.3).unwrap().1),
            tuple((
                tag_no_case("-> canvas"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| canvas(t.3).unwrap().1),
            tuple((
                tag_no_case("-> checklist"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| checklist(t.3).unwrap().1),
            tuple((
                tag_no_case("-> code"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| code(t.3).unwrap().1),
            tuple((
                tag_no_case("-> details"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| details(t.3).unwrap().1),
            tuple((
                tag_no_case("-> startdiv"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| startdiv(t.3).unwrap().1),
            tuple((
                tag_no_case("-> dlist"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| dlist(t.3).unwrap().1),
        )),
        alt((
            tuple((
                tag_no_case("-> h1"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| h1(t.3).unwrap().1),
            tuple((
                tag_no_case("-> h2"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| h2(t.3).unwrap().1),
            tuple((
                tag_no_case("-> h3"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| h3(t.3).unwrap().1),
            tuple((
                tag_no_case("-> h4"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| h4(t.3).unwrap().1),
            tuple((
                tag_no_case("-> h5"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| h5(t.3).unwrap().1),
            tuple((
                tag_no_case("-> h6"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| h6(t.3).unwrap().1),
            tuple((
                tag_no_case("-> hr"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| hr(t.3).unwrap().1),
            tuple((
                tag_no_case("-> image"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| image(t.3).unwrap().1),
            tuple((
                tag_no_case("-> list"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| list(t.3).unwrap().1),
            tuple((
                tag_no_case("-> menu"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| menu(t.3).unwrap().1),
            tuple((
                tag_no_case("-> nav"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| nav(t.3).unwrap().1),
            tuple((
                tag_no_case("-> notes"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| notes(t.3).unwrap().1),
            tuple((
                tag_no_case("-> note"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| note(t.3).unwrap().1),
        )),
        alt((
            tuple((
                tag_no_case("-> object"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| object(t.3).unwrap().1),
            tuple((
                tag_no_case("-> olist"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| olist(t.3).unwrap().1),
            tuple((
                tag_no_case("-> pre"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| pre(t.3).unwrap().1),
            tuple((
                tag_no_case("-> results"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| results(t.3).unwrap().1),
            tuple((
                tag_no_case("-> startcode"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> endcode"), rest)),
            ))
            .map(|t| code(t.3).unwrap().1),
            tuple((
                tag_no_case("-> subtitle"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| subtitle(t.3).unwrap().1),
            tuple((
                tag_no_case("-> table"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| table(t.3).unwrap().1),
            tuple((
                tag_no_case("-> textarea"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| textarea(t.3).unwrap().1),
            tuple((
                tag_no_case("-> title"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| title(t.3).unwrap().1),
        )),
        alt((
            tuple((
                tag_no_case("-> todos"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| todos(t.3).unwrap().1),
            tuple((
                tag_no_case("-> todo"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| todo(t.3).unwrap().1),
            tuple((
                tag_no_case("-> vimeo"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| vimeo(t.3).unwrap().1),
            tuple((
                tag_no_case("-> warning"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| warning(t.3).unwrap().1),
            tuple((
                tag_no_case("-> youtube"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| youtube(t.3).unwrap().1),
            tuple((
                tag_no_case("-> attributes"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| attributes(t.3).unwrap().1),
            tuple((
                tag_no_case("-> blurb"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| blurb(t.3).unwrap().1),
            tuple((
                tag_no_case("-> categories"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| categories(t.3).unwrap().1),
            tuple((
                tag_no_case("-> comment"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| comment(t.3).unwrap().1),
            tuple((
                tag_no_case("-> css"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| css(t.3).unwrap().1),
            tuple((
                tag_no_case("-> ext"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| ext(t.3).unwrap().1),
            tuple((
                tag_no_case("-> footnote"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| footnote(t.3).unwrap().1),
            tuple((
                tag_no_case("-> head"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| head(t.3).unwrap().1),
            tuple((
                tag_no_case("-> html"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| html(t.3).unwrap().1),
            tuple((
                tag_no_case("-> include"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| include(t.3).unwrap().1),
            tuple((
                tag_no_case("-> reference"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| reference(t.3).unwrap().1),
            tuple((
                tag_no_case("-> script"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| script(t.3).unwrap().1),
            tuple((
                tag_no_case("-> widget"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| widget(t.3).unwrap().1),
            tuple((
                tag_no_case("-> p"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| p(t.3).unwrap().1),
            tuple((
                tag_no_case("-> endcode"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| p(t.3).unwrap().1),
            tuple((
                tag_no_case("-> enddiv"),
                not_line_ending,
                line_ending,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|t| p(t.3).unwrap().1),
        )),
    ))(remainder)?;
    Ok((remainder, section))
}
