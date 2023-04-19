use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{char, digit1, space0},
    combinator::{map, opt, peek},
    error::{ParseError, VerboseError},
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated},
    Err, IResult,
};

use crate::ast::*;

pub type Result<I, O, E = VerboseError<I>> = IResult<I, O, E>;

pub fn page(input: &str) -> Result<&str, Page> {
    let (input, lines) = many0(line)(input)?;

    Ok((input, Page { lines }))
}

pub fn line(input: &str) -> Result<&str, Line> {
    if input.is_empty() {
        return Err(Err::Error(VerboseError::from_char(input, ' ')));
    }

    let (input, _) = opt(char('\n'))(input)?;
    let (input, list) = list(input)?;
    if let Some(list) = &list {
        map(many0(syntax), |c| {
            Line::new(
                LineKind::List(list.clone()),
                c.into_iter().flatten().collect(),
            )
        })(input)
    } else {
        map(many0(syntax), |c| {
            Line::new(LineKind::Normal, c.into_iter().flatten().collect())
        })(input)
    }
}

fn syntax(input: &str) -> Result<&str, Option<Syntax>> {
    map(
        alt((
            map(hashtag, |s| Syntax::new(SyntaxKind::HashTag(s))),
            map(block_quote, |s| Syntax::new(SyntaxKind::BlockQuote(s))),
            map(bracketing, |s| Syntax::new(SyntaxKind::Bracket(s))),
            map(external_link_plain, |s| {
                Syntax::new(SyntaxKind::Bracket(Bracket::new(
                    BracketKind::ExternalLink(s),
                )))
            }),
            map(text, |s| Syntax::new(SyntaxKind::Text(s))),
        )),
        Some,
    )(input)
}

// #tag
fn hashtag(input: &str) -> Result<&str, HashTag> {
    let terminators = vec![" ", "  ", "\n"];

    map(
        preceded(
            tag("#"),
            take_while(move |c: char| !terminators.contains(&c.to_string().as_str())),
        ),
        |s: &str| HashTag {
            value: s.to_string(),
        },
    )(input)
}

fn text(input: &str) -> Result<&str, Text> {
    if input.is_empty() {
        return Err(Err::Error(VerboseError::from_char(input, 'x')));
    }

    if input.starts_with('#') {
        return Err(Err::Error(VerboseError::from_char(input, ' ')));
    }

    fn take_until_tag(input: &str) -> Result<&str, &str> {
        let (input, _) = peek(take_until(" #"))(input)?;
        take_until("#")(input)
    }

    fn take_until_newline(input: &str) -> Result<&str, &str> {
        let (input, text) = take_until("\n")(input)?;
        Ok((input, text))
    }

    fn take_until_bracket(input: &str) -> Result<&str, &str> {
        take_while(|c| c != '[')(input)
    }

    let ret = vec![
        peek(take_until_tag)(input),
        peek(take_until_newline)(input),
        peek(take_until_bracket)(input),
    ];

    let ret = ret
        .iter()
        .filter(|r| r.is_ok())
        .filter_map(|x| x.as_ref().ok())
        .min_by(|(_, a), (_, b)| a.len().cmp(&b.len()));

    match ret {
        Some(&(input, consumed)) => {
            if consumed.is_empty() {
                return Err(Err::Error(VerboseError::from_char(input, ' ')));
            }
            let input = input.split_at(consumed.len()).1;
            let text = Text {
                value: consumed.to_string(),
            };
            Ok((input, text))
        }
        None => Err(Err::Error(VerboseError::from_char(input, ' '))),
    }
}

// []
fn bracketing(input: &str) -> Result<&str, Bracket> {
    let (input, _) = peek(delimited(char('['), take_while(|c| c != ']'), char(']')))(input)?;
    map(
        alt((
            map(emphasis, BracketKind::Emphasis),
            map(external_link, BracketKind::ExternalLink),
            map(internal_link, BracketKind::InternalLink),
        )),
        Bracket::new,
    )(input)
}

// [internal_link]
fn internal_link(input: &str) -> Result<&str, InternalLink> {
    let (input, text) = delimited(char('['), take_while(|c| c != ']'), char(']'))(input)?;
    Ok((input, InternalLink::new(text)))
}

fn external_link_plain(input: &str) -> Result<&str, ExternalLink> {
    let (input, protocol) = alt((tag("https://"), tag("http://")))(input)?;
    let (input, url) = take_until(" ")(input)?;
    Ok((
        input,
        ExternalLink::new(None, &format!("{}{}", protocol, url)),
    ))
}

// https://www.rust-lang.org/
// [https://www.rust-lang.org/]
// [https://www.rust-lang.org/ Rust]
// [Rust https://www.rust-lang.org/]
fn external_link(input: &str) -> Result<&str, ExternalLink> {
    fn url(input: &str) -> Result<&str, ExternalLink> {
        let (input, _) = opt(space0)(input)?;
        let (input, protocol) = alt((tag("https://"), tag("http://")))(input)?;
        let (input, url) = take_until("]")(input)?;
        Ok((
            input,
            ExternalLink::new(None, &format!("{}{}", protocol, url)),
        ))
    }

    fn url_title(input: &str) -> Result<&str, ExternalLink> {
        let (input, protocol) = alt((tag("https://"), tag("http://")))(input)?;
        let (input, url) = take_until("]")(input)?;
        let (input, _) = char(' ')(input)?;
        let (input, title) = take_until("]")(input)?;

        Ok((
            input,
            ExternalLink::new(Some(title), &format!("{}{}", protocol, url)),
        ))
    }

    fn title_url(input: &str) -> Result<&str, ExternalLink> {
        let (input, title) = take_until(" ")(input)?;
        let (input, _) = char(' ')(input)?;
        let (input, protocol) = alt((tag("https://"), tag("http://")))(input)?;
        let (input, url) = take_until("]")(input)?;

        Ok((
            input,
            ExternalLink::new(Some(title), &format!("{}{}", protocol, url)),
        ))
    }

    delimited(char('['), alt((url_title, title_url, url)), char(']'))(input)
}

// fn image() {}

// fn icon() {}

// [*-/** emphasis]
// [[Bold]] or [* Bold] or [*** Bold]
// [/ italic]
// [- strikethrough]
fn emphasis(input: &str) -> Result<&str, Emphasis> {
    let (input, text) = delimited(char('['), take_while(|c| c != ']'), char(']'))(input)?;
    let (rest, tokens) = take_while(|c| ['*', '/', '-'].contains(&c))(text)?;
    let (text, _) = char(' ')(rest)?;

    let mut bold = 0;
    let mut italic = 0;
    let mut strikethrough = 0;
    for c in tokens.chars() {
        match &c {
            '*' => bold += 1,
            '/' => italic += 1,
            '-' => strikethrough += 1,
            _ => {}
        }
    }

    Ok((input, Emphasis::new(text, bold, italic, strikethrough)))
}

// fn bold() {}

// fn italic() {}

// fn strilethrough() {}

// fn math() {}

// `block_quote`
fn block_quote(input: &str) -> Result<&str, BlockQuote> {
    map(
        delimited(char('`'), take_while(|c| c != '`'), char('`')),
        BlockQuote::new,
    )(input)
}

// fn code_block() {}

// fn table() {}

// fn quote() {}

// fn commandline() {}

// fn helpfeel() {}

// <tab>
// <tab>1.
fn list(input: &str) -> Result<&str, Option<List>> {
    let (input, tabs) = opt(many1(char('\t')))(input)?;
    let (input, decimal) = opt(terminated(digit1, tag(". ")))(input)?;
    if let Some(tabs) = tabs {
        let kind = match &decimal {
            Some(_) => ListKind::Decimal,
            None => ListKind::Disc,
        };
        Ok((
            input,
            Some(List {
                level: tabs.len(),
                kind,
            }),
        ))
    } else {
        Ok((input, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn hashtag_test() {
        assert_eq!(hashtag("#tag"), Ok(("", HashTag::new("tag"))));
        assert_eq!(hashtag("#tag"), Ok(("", HashTag::new("tag"))));
        assert_eq!(hashtag("#tag\n"), Ok(("\n", HashTag::new("tag"))));
        assert_eq!(hashtag("#tag\n"), Ok(("\n", HashTag::new("tag"))));
        assert_eq!(hashtag("#tag "), Ok((" ", HashTag::new("tag"))));
        assert_eq!(hashtag("#tag "), Ok((" ", HashTag::new("tag"))));
        assert_eq!(hashtag("#tag  "), Ok(("  ", HashTag::new("tag"))));
        assert_eq!(hashtag("#tag  "), Ok(("  ", HashTag::new("tag"))));
        assert_eq!(hashtag("####tag"), Ok(("", HashTag::new("###tag"))));
        assert_eq!(hashtag("####tag"), Ok(("", HashTag::new("###tag"))));
        assert_eq!(hashtag("#[tag"), Ok(("", HashTag::new("[tag"))));
        assert_eq!(hashtag("#[tag"), Ok(("", HashTag::new("[tag"))));
        // assert!(hashtag("#[tag]").is_err());
        // assert!(hashtag("#[tag]").is_err());
        // assert!(hashtag("# tag").is_err());
        // assert!(hashtag("# tag").is_err());
    }

    #[test]
    fn emphasis_test() {
        assert_eq!(
            emphasis("[* text]"),
            Ok(("", Emphasis::bold_level("text", 1)))
        );
        assert_eq!(
            emphasis("[***** text]"),
            Ok(("", Emphasis::bold_level("text", 5)))
        );
        assert_eq!(emphasis("[/ text]"), Ok(("", Emphasis::italic("text"))));
        assert_eq!(
            emphasis("[- text]"),
            Ok(("", Emphasis::strikethrough("text")))
        );
        assert_eq!(
            emphasis("[*/*-* text]"),
            Ok(("", Emphasis::new("text", 3, 1, 1)))
        );
    }

    #[test]
    fn test_block_quote() {
        assert!(block_quote("123abc").is_err());
        assert!(block_quote("`123abc").is_err());
        assert_eq!(block_quote("`code`"), Ok(("", BlockQuote::new("code"))));
        assert_eq!(
            block_quote("`code` test"),
            Ok((" test", BlockQuote::new("code")))
        );
    }
}
