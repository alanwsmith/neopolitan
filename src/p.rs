use crate::block::Block;
use nom::IResult;

pub fn p(_source: &str) -> IResult<&str, Block> {
    Ok((
        "",
        Block::P {
            attributes: None,
            children: None,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Block;
    #[test]
    fn basic_title_response() {
        let source = "";
        let expected = Ok((
            "",
            Block::P {
                attributes: None,
                children: None,
            },
        ));
        let result = p(source);
        assert_eq!(expected, result);
    }
}
