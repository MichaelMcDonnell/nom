extern crate nom;

use nom::bytes::complete::tag;
use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::sequence::tuple;
use nom::Err::Error;
use nom::Finish;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum CustomError<I> {
  MyError,
  Nom(I, ErrorKind),
}

impl<I> ParseError<I> for CustomError<I> {
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    CustomError::Nom(input, kind)
  }

  fn append(_: I, _: ErrorKind, other: Self) -> Self {
    other
  }
}

fn foo(input: &str) -> IResult<&str, &str, CustomError<&str>> {
  let (i, o) = tag("foo")(input)?;
  Ok((i, o))
}

fn bar(input: &str) -> IResult<&str, &str, CustomError<&str>> {
  /*
  let (i, o) = tag::<_, _, CustomError<_>>("bar")(input)
    .map_err(|_e| nom::Err::Error(CustomError::MyError))?;
  Ok((i, o))
  */
  match tag::<_, _, CustomError<_>>("bar")(input) {
    Ok((i, o)) => Ok((i, o)),
    _ => Err(Error(CustomError::MyError)),
  }
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
    Err(CustomError::MyError) => println!("Got MyError"),
    Err(CustomError::Nom(s, error_kind)) => {
      println!("Got Nom error: '{}', '{:?}'", s, error_kind)
    }
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
