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

fn main() {
    let input_alfa = fs::read_to_string(
        "_test_files/scratchpad/alfa.neo",
    )
    .unwrap();

    let tokens: Vec<_> =
        Token::lexer(&input_alfa).spanned().collect();

    for token in tokens.iter() {
        dbg!(&token);
    }

    // dbg!(input_alfa);

    println!("Hello, world!");
}
