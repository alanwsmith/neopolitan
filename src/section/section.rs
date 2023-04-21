use crate::block::block::*;






// AUTO GENERATED START: calls //
use crate::section::aside::*;
use crate::section::blockquote::*;
use crate::section::canvas::*;
use crate::section::checklist::*;
use crate::section::code::*;
use crate::section::details::*;
use crate::section::div::*;
use crate::section::dlist::*;
use crate::section::figure::*;
use crate::section::h1::*;
use crate::section::h2::*;
use crate::section::h3::*;
use crate::section::h4::*;
use crate::section::h5::*;
use crate::section::h6::*;
use crate::section::image::*;
use crate::section::list::*;
use crate::section::menu::*;
use crate::section::nav::*;
use crate::section::note::*;
use crate::section::notes::*;
use crate::section::object::*;
use crate::section::olist::*;
use crate::section::p::*;
use crate::section::picture::*;
use crate::section::pre::*;
use crate::section::results::*;
use crate::section::startcode::*;
use crate::section::subtitle::*;
use crate::section::table::*;
use crate::section::textarea::*;
use crate::section::title::*;
use crate::section::todo::*;
use crate::section::vimeo::*;
use crate::section::warning::*;
use crate::section::youtube::*;
use crate::section::video::*;
// AUTO GENERATED END: calls //




use crate::section::section_attributes::SectionAttribute;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Section {
    




// AUTO GENERATED START: enum //
AsideSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
BlockquoteSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
CanvasSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
ChecklistSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
CodeSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
DetailsSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
DivSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
DescriptionListSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
FigureSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
H1Section
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
H2Section
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
H3Section
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
H4Section
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
H5Section
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
H6Section
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
ImageSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
ListSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
MenuSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
NavSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
NoteSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
NotesSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
ObjectSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
OrderedListSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
ParagraphsSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
PictureSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
PreSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
ResultsSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
CodeStartEndSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
SubtitleSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
TableSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
TextareaSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
TitleSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
TodoSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
VimeoSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
WarningSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
YouTubeSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
VideoSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
// AUTO GENERATED END: enum //







    Placeholder,
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = multispace0(source)?;
    let (remainder, section) = 
    
    alt((
    
    
    alt((
        






// AUTO GENERATED START: tags //
tuple((
            tag("-> aside"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| aside(t.3).unwrap().1),
tuple((
            tag("-> blockquote"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| blockquote(t.3).unwrap().1),
tuple((
            tag("-> canvas"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| canvas(t.3).unwrap().1),
tuple((
            tag("-> checklist"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| checklist(t.3).unwrap().1),
tuple((
            tag("-> code"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| code(t.3).unwrap().1),
tuple((
            tag("-> details"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| details(t.3).unwrap().1),
tuple((
            tag("-> div"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| div(t.3).unwrap().1),
tuple((
            tag("-> dlist"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| dlist(t.3).unwrap().1),
tuple((
            tag("-> figure"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| figure(t.3).unwrap().1),


    )),

    alt((
        






tuple((
            tag("-> h1"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| h1(t.3).unwrap().1),
tuple((
            tag("-> h2"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| h2(t.3).unwrap().1),
tuple((
            tag("-> h3"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| h3(t.3).unwrap().1),
tuple((
            tag("-> h4"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| h4(t.3).unwrap().1),
tuple((
            tag("-> h5"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| h5(t.3).unwrap().1),
tuple((
            tag("-> h6"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| h6(t.3).unwrap().1),
tuple((
            tag("-> image"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| image(t.3).unwrap().1),
tuple((
            tag("-> list"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| list(t.3).unwrap().1),
tuple((
            tag("-> menu"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| menu(t.3).unwrap().1),
tuple((
            tag("-> nav"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| nav(t.3).unwrap().1),
tuple((
            tag("-> note"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| note(t.3).unwrap().1),
tuple((
            tag("-> notes"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| notes(t.3).unwrap().1),


    )),

    alt((
        




tuple((
            tag("-> object"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| object(t.3).unwrap().1),
tuple((
            tag("-> olist"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| olist(t.3).unwrap().1),
tuple((
            tag("-> p"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| p(t.3).unwrap().1),
tuple((
            tag("-> picture"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| picture(t.3).unwrap().1),
tuple((
            tag("-> pre"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| pre(t.3).unwrap().1),
tuple((
            tag("-> results"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| results(t.3).unwrap().1),
tuple((
            tag("-> startcode"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| startcode(t.3).unwrap().1),
tuple((
            tag("-> subtitle"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| subtitle(t.3).unwrap().1),
tuple((
            tag("-> table"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| table(t.3).unwrap().1),
tuple((
            tag("-> textarea"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| textarea(t.3).unwrap().1),
tuple((
            tag("-> title"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| title(t.3).unwrap().1),


    )),

    alt((
        



        
tuple((
            tag("-> todo"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| todo(t.3).unwrap().1),
tuple((
            tag("-> vimeo"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| vimeo(t.3).unwrap().1),
tuple((
            tag("-> warning"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| warning(t.3).unwrap().1),
tuple((
            tag("-> youtube"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| youtube(t.3).unwrap().1),
tuple((
            tag("-> video"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| video(t.3).unwrap().1),
// AUTO GENERATED END: tags //






))
    

))
    
    
    
    (remainder)?;
    Ok((remainder, section))
}
