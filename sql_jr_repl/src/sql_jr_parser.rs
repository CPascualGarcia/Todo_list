use nom::{LocatedSpan,IResult};
use nom::bytes::complete::take_while1;
use nom::combinator::map;
use nom::bytes::complete::tag_no_case;

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
use nom::error::context; // Provides extra information in case of error


// Data types supported in the database
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
            alt((
                map(tag_no_case("string"), |_| Self::String),
                map(tag_no_case("int"), |_| Self::Int),
            )),
        )(input)
    }
}

// Struct where we store the information in a column
#[derive(Debug,Clone,Eq,Hash,PartialEq,Serialize,Deserialize)]
pub struct Column {
    pub name: String,
    pub type_info: sqlTypeinfo,
}

// Parse "<colName> <colType>"
impl <'a> Parse<'a> for Column {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        // Using context will pull later a better error message
        context("Create column",
            map(
                separated_pair(identifier.context("Column name"),multispace1, sqlTypeinfo::parse),
                |(name, type_info)| Self {name, type_info},
            ),
        )(input)
    }
}

pub(crate) fn comma_sep<I, O, E, F>(
    f: F,
) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
{
    separated_list1(tuple((multispace0, char(','), multispace0)), f)
}

// Struct of the table provided its columns
#[derive(Debug,Clone,Eq,Hash,PartialEq,Serialize,Deserialize)]
pub struct CreateStatement {
    pub table: String,
    pub columns: Vec<Column>,
}

// List of column definitions
fn column_definitions(input: RawSpan<'_>) -> ParseResult<'_, Vec<Column>> {
    context(
        "Column definitions",
        map(tuple((char('('), comma_sep(Column::parse), char(')'))), 
        |(_, columns, _)| columns,
        ),
    )(input)
}












