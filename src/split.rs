use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::IResult;

pub fn split<'a>(source: &'a str, separator: &'a str) -> IResult<&'a str, Vec<&'a str>> {
    let (remainder, _) = opt(tag(separator))(source)?;
    let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
    Ok(("", items))
}

#[cfg(test)]
mod tests {
    use crate::split::split;

    #[test]
    fn test_split() {
        assert_eq!(split("a|b|c", "|"), Ok(("", vec!["a", "b", "c"])));
        assert_eq!(split("a|b|c|", "|"), Ok(("", vec!["a", "b", "c"])));
        assert_eq!(split("|a|b|c", "|"), Ok(("", vec!["a", "b", "c"])));
        assert_eq!(split("a", "|"), Ok(("", vec!["a"])));
    }
}
