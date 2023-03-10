#![allow(dead_code)]
use logos::Logos;
use std::fs;

#[derive(Logos, Debug, PartialEq)]
enum Token<'a> {
    #[regex(r"(?i)-> TITLE")]
    TitleTag,

    #[regex(r"(?i)-> T[^I]")]
    TextTag,

    #[regex(r" ")]
    Space,

    #[regex(r"\n")]
    Newline,

    #[regex(r"\w+")]
    Word(&'a str),

    #[error]
    Error,
}

#[derive(Debug)]
struct Node {
    kind: Kind,
    children: Vec<Node>,
}

#[derive(Debug)]
enum Kind {
    Title(Title),
    Paragraph(Paragraph),
}

#[derive(Debug)]
struct Page {
    content: Vec<Content>,
}

#[derive(Debug)]
enum Content {
    Title(Title),
    Paragraph(Paragraph),
}

#[derive(Debug)]
struct Title {}

#[derive(Debug)]
struct Paragraph {}

fn parseit(lex: &mut dyn Iterator<Item = Token>) {
    dbg!(lex.next());
    dbg!(lex.next());
    dbg!(lex.next());
    dbg!(lex.next());
    println!("HEREREERR");
}

fn main() {
    println!("Starting");

    let input_alfa = fs::read_to_string(
        "_test_files/scratchpad/alfa.neo",
    )
    .unwrap();

    let mut lex = Token::lexer(&input_alfa).peekable();
    parseit(&mut lex);

    // while let Some(x) = lex.next() {
    //     dbg!(x);
    //     dbg!(lex.peek());
    // }

    // dbg!(lex.peek().unwrap());
    // dbg!(lex.next());
    // dbg!(lex.peek());

    // dbg!(tokens);
    // let current_index: usize = 0;
    // process_token(tokens, current_index);

    // let mut p = Page { content: vec![] };

    // for token in tokens.iter() {
    //     // p.content.push(token)
    //     // dbg!(&token);
    // }
    // dbg!(p);

    println!("Done.");
}

// fn process_token<T: std::fmt::Debug>(
//     tokens: Vec<T>, current_index: usize,
// ) {
//     if current_index < tokens.len() {
//         let token = tokens.get(current_index);
//         let thing = token.unwrap();
//         dbg!(thing);
//         match thing {
//             Token::TitleTag => println!("HERERER"),
//             _ => println!("HEREEQWEQWE"),
//         }
//         let next_index = current_index + 1;
//         process_token(tokens, next_index);
//     }
//     // }
//     // else {
//     //     println!("done.");
//     // }
// }
