use crate::*;

use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    branch::alt,
    character::{
        complete::{one_of, space0, digit1},
    },
    multi::{separated_list, many1},
    sequence::{pair, preceded, delimited},
};

pub fn query(input : &str) -> IResult<&str, Query> {
    alt((add, done, search))(input)
}

fn ws(input : &str) -> IResult<&str, char> { 
    one_of(" \t")(input)
}

fn add(input : &str) -> IResult<&str, Query> {
    match preceded(
        pair(tag("add"), ws),
        pair(description, preceded(space0, tags))
    )(input) {
        Err(e) => Err(e),
        Ok( (rest, (d, ts)) ) => Ok((
            rest,
            Query::Add(d, ts)
        )),
    }
}

fn is_lowecase_or_dash(c : char) -> bool {
    c.is_ascii_lowercase() || c == '-'
}

fn sentence(input : &str) -> IResult<&str, Vec<Word>> {
    match separated_list(ws, word)(input) {
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

fn todo_tag(input : &str) -> IResult<&str, &str> {
    preceded(tag("#"), word)(input)
}

fn description(input : &str) -> IResult<&str, Vec<Word>> {
    match delimited(tag("\""), sentence, tag("\""))(input) {
        Err(e) => Err(e),
        Ok((rest, d)) => Ok((rest, d)),
    }
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

fn done(input : &str) -> IResult<&str, Query> {
    match preceded(
        pair(tag("done"), ws),
        many1(digit1)
    )(input) {
        Err(e) => Err(e),
        Ok((rest, i)) => Ok((
            rest,
            Query::Done(Index::new(vec_to_u64(i)))
        ))
    }
}

fn vec_to_u64(dss : Vec<&str>) -> u64 {
    let ds = dss.iter().fold("".to_string(), |acc, x| format!("{}{}", acc, x));
    ds.parse::<u64>().unwrap()
}

fn search(input : &str) -> IResult<&str, Query> {
    match preceded(pair(tag("search"), ws),
        separated_list(
            tag(" "),
            search_word_or_tag
        )
    )(input) {
        Err(e) => Err(e),
        Ok((rest, mash)) => Ok((rest, mash_to_query(mash))),
    }
}

fn search_word_or_tag(input : &str) -> IResult<&str, SearchWordOrTag> {
    match alt((pair(tag("#"), word), (pair(tag(""), word))))(input) { //TODO: ensure that tag("") does what I think it does
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

fn mash_to_query(mash : Vec<SearchWordOrTag>) -> Query {
    Query::Search(SearchParams{
        params: mash,
    })
}