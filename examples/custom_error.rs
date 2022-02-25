extern crate nom;

use nom::bytes::complete::tag;
use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::sequence::tuple;
use nom::Finish;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Context {
  Foo,
  Bar,
  Nom,
}

#[derive(Debug, PartialEq)]
pub struct CustomError<I> {
  context: Context,
  input: I,
  kind: ErrorKind,
}

impl<I> ParseError<I> for CustomError<I> {
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    CustomError {
      context: Context::Nom,
      input,
      kind,
    }
  }

  fn append(_: I, _: ErrorKind, other: Self) -> Self {
    other
  }
}

pub trait CustomContext<I>: Sized {
  fn context(_input: I, _context: Context, other: Self) -> Self {
    other
  }
}

impl<I> CustomContext<I> for CustomError<I> {
  fn context(input: I, context: Context, other: Self) -> Self {
    Self {
      context,
      input: input,
      kind: other.kind,
    }
  }
}

pub fn context<I: Clone, E: CustomContext<I>, F, O>(
  context: Context,
  mut f: F,
) -> impl FnMut(I) -> IResult<I, O, E>
where
  F: nom::Parser<I, O, E>,
{
  move |i: I| match f.parse(i.clone()) {
    Ok(o) => Ok(o),
    Err(nom::Err::Incomplete(i)) => Err(nom::Err::Incomplete(i)),
    Err(nom::Err::Error(e)) => Err(nom::Err::Error(E::context(i, context, e))),
    Err(nom::Err::Failure(e)) => Err(nom::Err::Failure(E::context(i, context, e))),
  }
}

fn foo(input: &str) -> IResult<&str, &str, CustomError<&str>> {
  let (i, o) = tag("foo")(input)?;
  Ok((i, o))
}

fn bar(input: &str) -> IResult<&str, &str, CustomError<&str>> {
  let (i, o) = context(Context::Bar, tag("bar"))(input)?;
  Ok((i, o))
}

#[derive(Debug)]
struct FooBar<'a>(&'a str, &'a str);

fn parse(input: &str) -> IResult<&str, FooBar, CustomError<&str>> {
  let (i, (foo, bar)) = tuple((foo, bar))(input)?;

  Ok((i, FooBar(foo, bar)))
}

fn main() {
  let res = parse("foo42bar").finish();
  match res {
    Ok((i, foo_bar)) => println!(
      "Input is '{}' and output is ({}, {})",
      i, foo_bar.0, foo_bar.1
    ),
    Err(CustomError {
      context,
      input,
      kind,
    }) => println!(
      "Got {:?} with input {:?} and nom kind {:?}",
      context, input, kind
    ),
  };
}

#[cfg(test)]
mod tests {
  use super::parse;
  use super::CustomError;
  use nom::Err::Error;

  #[test]
  fn it_works() {
    let err = parse("foo42bar").unwrap_err();
    match err {
      Error(e) => assert_eq!(e, CustomError::MyError),
      _ => panic!("Unexpected error: {:?}", err),
    }
  }
}
