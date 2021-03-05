use crate::*;

use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    branch::alt,
    character::{
        complete::{one_of, digit1},
    },
    multi::{separated_list, separated_nonempty_list, many1},
    sequence::{pair, preceded, delimited},
};

//Specification parsers

/*Assumptions:
-Whitespace in specification grammar consists of a single space or tab character (as parsed by the 'ws' function below) 
-Descriptions consist of at least one word
*/

pub fn query(input : &str) -> IResult<&str, Query> {
    alt((add, done, search))(input)
}

fn add(input : &str) -> IResult<&str, Query> {
    match preceded(
        pair(tag("add"), ws),
        pair(delimited(tag("\""), description, tag("\"")), preceded(ws, tags))
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
    match separated_nonempty_list(tag(" "), word)(input) {
        Err(e) => Err(e),
        Ok((rest, ts)) => Ok((
            rest,
            ts.iter().map(|w| Word::new(w)).collect()
        )),
    }
}

fn word(input : &str) -> IResult<&str, &str> {
    take_while1(is_lowecase_or_dash)(input)
}

fn tags(input : &str) -> IResult<&str, Vec<Tag>> {
    match separated_list(ws, todo_tag)(input) {
        Err(e) => Err(e),
        Ok((rest, ts)) => Ok((
            rest,
            ts.iter().map(|w| Tag::new(w)).collect()
        )),
    }
}

fn todo_tag(input : &str) -> IResult<&str, &str> {
    preceded(tag("#"), word)(input)
}

fn index(input : &str) -> IResult<&str, Index> {
    many1(digit1)(input).map(|(rest, v)| (rest, Index::new(vec_to_u64(v))))
}

fn search_query(input : &str) -> IResult<&str, SearchParams> {
    separated_nonempty_list(tag(" "), search_word_or_tag)(input).map(|(rest, p)| (rest, SearchParams{params: p}))
}

//Helper parsers and functions
fn search_word_or_tag(input : &str) -> IResult<&str, SearchWordOrTag> {
    match alt((pair(tag("#"), word), (pair(tag(""), word))))(input) { //TODO: ensure tag("") succeeds on all inputs and parses exactly nothing
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

fn ws(input : &str) -> IResult<&str, char> { 
    one_of(" \t")(input)
}

fn is_lowecase_or_dash(c : char) -> bool {
    c.is_ascii_lowercase() || c == '-'
}

fn vec_to_u64(dss : Vec<&str>) -> u64 {
    let ds = dss.iter().fold("".to_string(), |acc, x| format!("{}{}", acc, x));
    ds.parse::<u64>().unwrap()
}