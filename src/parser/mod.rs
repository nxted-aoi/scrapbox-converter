use std::convert::identity;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{char, space0},
    combinator::{map, opt, peek},
    error::{ParseError, VerboseError},
    multi::many0,
    sequence::{delimited, preceded},
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
    map(many0(syntax), |c| {
        Line::new(
            LineKind::Normal,
            c.into_iter().filter_map(identity).collect(),
        )
    })(input)
}

fn syntax(input: &str) -> Result<&str, Option<Syntax>> {
    map(
        alt((
            map(hashtag, |s| Syntax::new(SyntaxKind::HashTag(s))),
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

    if input.starts_with("#") {
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
            return Ok((input, text));
        }
        None => {
            return Err(Err::Error(VerboseError::from_char(input, ' ')));
        }
    }
}

// []
fn bracketing(input: &str) -> Result<&str, Bracket> {
    let (input, _) = peek(delimited(char('['), take_while(|c| c != ']'), char(']')))(input)?;
    map(
        alt((
            map(decoration, |c| BracketKind::Decoration(c)),
            map(external_link, |c| BracketKind::ExternalLink(c)),
            map(internal_link, |c| BracketKind::InternalLink(c)),
        )),
        |kind| Bracket::new(kind),
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

fn image() {}

fn icon() {}

fn decoration(input: &str) -> Result<&str, Decoration> {
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

    Ok((input, Decoration::new(text, bold, italic, strikethrough)))
}

fn bold() {}

fn italic() {}

fn strilethrough() {}

fn math() {}

fn block_quote() {}

fn code_block() {}

fn table() {}

fn quote() {}

fn commandline() {}

fn helpfeel() {}

fn bullet_points() {}
