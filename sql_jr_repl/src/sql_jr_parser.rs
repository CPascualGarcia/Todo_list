use nom::{LocatedSpan,IResult};

// Define the data type RawSpan for a span of a string
// LocatedSpan represents a span of text with a memory location
pub type RawSpan<'a> =  LocatedSpan<&'a str>;

// the result for all of our parsers, they will have our span type as input and can have any output
// this will use a default error type but we will change that latter

// IResult is the type returned by nom after the parsing.
// Here, <'a> is the input type, and T the type of the parsed value
pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T>;

// Parse an SQL identifier from a string
pub crate fn identifier(i:RawSpan) -> ParseResult<String> {
    map(
        take_while1(|c: char| c.is_alphanumeric()), 
                    |s: RawSpan| {s.fragment().to_string(), String::from})(i)
}

pub trait Parse<'a>: Sized {

    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self>;
    fn parse_from_raw(input: &'a str) -> ParseResult<'a, Self> {
        let i = LocatedSpan::new(input);
        Self::parse(i)
    }
}


// Using tag_no_case from nom_supreme since its error is nicer
// ParserExt is mostly for adding `.context` on calls to identifier to say what kind of identifier we want
use nom_supreme::{tag::complete::tag_no_case, ParserExt};
// many other imports omitted

#[derive(Debug,Clone,Eq,Hash,PartialEq,Serialize,Deserialize)]
pub enum sqlTypeinfo {
    String,
    Int,
}

// Parse "string | int"
impl <'a> Parse<'a> for sqlTypeinfo {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        // Using context will pull later a better error message
        context("Column Type",
        
        )(input)
    }
}