use nom::{
    character::complete::char,
    combinator::{map, opt},
    error::{ParseError, VerboseError},
    multi::many0,
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
    map(many0(syntax), |c| Line {
        items: c.into_iter().filter_map(identify).collect(),
    })(input)
}

fn syntax() {}
