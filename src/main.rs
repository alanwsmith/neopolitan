use nom::IResult;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::multi::many1;
use nom::multi::many_till;
use nom::bytes::complete::tag;
use nom::Parser;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_until1;
use nom::combinator::rest;
use nom::combinator::eof;
use nom::character::complete::not_line_ending;

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
    Paragraphs { children: Vec<Wrapper>},
    H3
}

#[derive(Debug)]
enum Wrapper {
    Paragraph { children: Vec<Content> } 
}

// #[derive(Debug)]
// struct Paragraph {
// }

#[derive(Debug)]
enum Content {
    PlainText { value: String }
}


#[derive(Debug)]
struct Title {
    children: Vec<Content>,
    attributes: Vec<(String, String)>
}


fn title(data: &str) -> IResult<&str, Section> {
    let (data, content) = text(data)?;
    // println!("{}", data);
    Ok((data, Section::Title{children: vec![content]}))
}


fn text(data: &str) -> IResult<&str, Content> {
    let (data, _) = multispace0(data)?;
    let (data, content) = not_line_ending(data)?;
    Ok((data, Content::PlainText { value: content.trim().to_string()}))
}

fn paragraph(data: &str) -> IResult<&str, Wrapper> {
     // println!("---------------");
     // println!("{}", data);
     //

      let (data, _) = multispace0(data)?;
     let (data, content) = 
         alt((take_until("\n\n"), rest))(data)?;


        // println!("{}", data);
        // println!("-------- {}", content);


      // let (data, _) = opt(tag("\n\n"))(data)?;
     // println!("{}", data);
     // // println!("{}", data);
      let (_, content) = text(content)?;

    // println!("{:?}", content);
    //    println!("{}", data);



      // Ok((data, Content::PlainText { value: content.to_string()}))
        Ok((data.trim(), Wrapper::Paragraph{ children: vec![
           content
        ] }))

}

fn paragraphs(data: &str) -> IResult<&str, Section> {

     // let (data, _) = multispace0(data)?;


       let (data, content) = many_till(paragraph, eof)(data)?;
        dbg!(&content);
       // dbg!(&data);

    // println!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");

    // let mut paras: Vec<Wrapper> = vec![];
    // for para in data.split("\n\n") {
    //     paras.push(Wrapper::Paragraph { children: vec![] });
    // }

    Ok((data, Section::Paragraphs{children: vec![] }))
}


fn section(data: &str) -> IResult<&str, Section> {
    // println!("============");
    let (data, _) = multispace0(data)?;
     // println!("{}", data);
    // println!("------------");
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
         Marker::Paragraphs => Ok((data, paragraphs(content).unwrap().1)),
         Marker::H3 => Ok((data, Section::H3)),
     }
}


// #[derive[Debug])
fn parse(data: &str) -> IResult<&str, Vec<Section>> {
    let (data, _) = multispace0(data)?;
    // dbg!(data);
    // dbg!(sections);
    // println!("here");
    // Ok(("", ""))
    // let (data, sections) = 
        many1(section)(data)
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
