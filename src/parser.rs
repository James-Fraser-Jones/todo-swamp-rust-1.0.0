use crate::*;

use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    branch::alt,
    character::{
        complete::{one_of, digit1},
    },
    multi::{separated_nonempty_list, many0},
    sequence::{pair, preceded, delimited},
};

//Specification parsers

/*Assumptions:
-Whitespace within specification grammar consists of a single space or tab character (as parsed by the 'ws' function below)
-Whitespace is not required following a <description> if the add query's list of <tags> is empty
-<description>s consist of at least one <word>
*/

pub fn query(input : &str) -> IResult<&str, Query> {
    alt((add, done, search))(input)
}

fn add(input : &str) -> IResult<&str, Query> {
    match preceded(
        pair(tag("add"), ws),
        pair(delimited(tag("\""), description, tag("\"")), many0(preceded(ws, todo_tag)))
    )(input) {
        Err(e) => Err(e),
        Ok((rest, (d, ts))) => Ok((rest, Query::Add(d, ts))),
    }
}

fn done(input : &str) -> IResult<&str, Query> {
    match preceded(
        pair(tag("done"), ws),
        index
    )(input) {
        Err(e) => Err(e),
        Ok((rest, i)) => Ok((rest, Query::Done(i))),
    }
}

fn search(input : &str) -> IResult<&str, Query> {
    match preceded(
        pair(tag("search"), ws),
        search_query
    )(input) {
        Err(e) => Err(e),
        Ok((rest, p)) => Ok((rest, Query::Search(p))),
    }
}

fn description(input : &str) -> IResult<&str, Vec<Word>> {
    separated_nonempty_list(tag(" "), word)(input)
}

fn word(input : &str) -> IResult<&str, Word> {
    prim_word(input).map(|(rest, w)| (rest, Word::new(w)))
}

fn todo_tag(input : &str) -> IResult<&str, Tag> {
    preceded(tag("#"), prim_word)(input).map(|(rest, w)| (rest, Tag::new(w)))
}

fn index(input : &str) -> IResult<&str, Index> {
    digit1(input).map(|(rest, v)| (rest, Index::new(v.parse().unwrap())))
}

fn search_query(input : &str) -> IResult<&str, SearchParams> {
    separated_nonempty_list(tag(" "), search_word_or_tag)(input).map(|(rest, p)| (rest, SearchParams{params: p}))
}

//Helper parsers and functions
fn search_word_or_tag(input : &str) -> IResult<&str, SearchWordOrTag> {
    match alt((pair(tag("#"), prim_word), pair(tag(""), prim_word)))(input) {
        Err(e) => Err(e),
        Ok((rest, (hash, wot))) => {
            if hash.starts_with("#") {
                Ok( (rest, SearchWordOrTag::RawTag(wot.to_string())) )
            } else {
                Ok( (rest, SearchWordOrTag::RawWord(wot.to_string())) )
            }
        }
    }
}
fn prim_word(input : &str) -> IResult<&str, &str> {
    take_while1(is_lowecase_or_dash)(input)
}
fn is_lowecase_or_dash(c : char) -> bool {
    c.is_ascii_lowercase() || c == '-'
}
fn ws(input : &str) -> IResult<&str, char> { 
    one_of(" \t")(input)
}
