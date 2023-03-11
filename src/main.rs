#![allow(warnings)]
use nom::IResult;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::multi::many1;
use nom::bytes::complete::tag;
use nom::Parser;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_until1;
use nom::combinator::rest;

#[derive(Debug)]
struct Page {
  // sections: Vec<Section>
}


#[derive(Debug)]
enum Marker {
    Title, 
        Paragraphs,
        H3
}

#[derive(Debug)]
enum Section {
    Title { children: Vec<Content>}, 
    Paragraphs,
    H3
}

#[derive(Debug)]
enum Content {
    Text
}

#[derive(Debug)]
struct Title {
    children: Vec<Content>,
    attributes: Vec<(String, String)>
}


fn title(data: &str) -> IResult<&str, Section> {
    println!("{}", data);
    Ok((data, Section::Title{children: vec![]}))
}


fn section(data: &str) -> IResult<&str, Section> {
    println!("============");
    let (data, _) = multispace0(data)?;
     println!("{}", data);
    println!("------------");
     let (data, section_type) = alt((
            tag("-> P").map(|_| Marker::Paragraphs),
            tag("-> TITLE").map(|_| Marker::Title),
            tag("-> H3").map(|_| Marker::H3),
            ))(data)?;
     
     // println!("{}", data);
     let (data, content) = alt((
        take_until1("\n\n-> "), 
        rest
        ))(data)?;

     // println!("{}", content.trim());

     match section_type {
         Marker::Title => Ok((data, title(content).unwrap().1)),
         Marker::H3 => Ok((data, Section::H3)),
         Marker::Paragraphs => Ok((data, Section::Paragraphs))
     }
         
}

// #[derive[Debug])
fn parse(data: &str) -> IResult<&str, &str> {
    let (data, _) = multispace0(data)?;
    let (data, sections) = many1(section)(data)?;
    // dbg!(data);
    dbg!(sections);
    println!("here");
    Ok(("", ""))
}

fn main() {
    let data = r#"-> TITLE

This is the title 

-> P

This is line one

This is line two

-> H3

This is the other line

"#;

    let results = parse(data);
    dbg!(results);
}
