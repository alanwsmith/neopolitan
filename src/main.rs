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
enum Section{
    Title,
    P,
    H3
}

fn section(data: &str) -> IResult<&str, Section> {
    println!("------------");
    let (data, _) = multispace0(data)?;
    // println!("{}", data);
     let (data, section_type) = alt((
            tag("-> P").map(|_| Section::P),
            tag("-> TITLE").map(|_| Section::Title),
            tag("-> H3").map(|_| Section::H3),
             // tag("-> TITLE\n\n"),
             // tag("-> P\n\n"),
            ))(data)?;
     
     // println!("{}", data);
     let (data, content) = alt((
        take_until1("\n\n-> "), 
        rest
        ))(data)?;

 println!("{}", content);


     match section_type {
         Section::Title => Ok((data, Section::Title)),
         Section::H3 => Ok((data, Section::H3)),
         Section::P => Ok((data, Section::P))
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

This is the TITLE

-> P

This is line one

This is line two

-> H3

This is the other line

"#;

    let results = parse(data);
    dbg!(results);
}
