use crate::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{digit1, space0, space1},
    combinator::opt,
    multi::{many1, separated_list},
    sequence::{delimited, pair, preceded},
    IResult,
};

pub fn query(input: &str) -> IResult<&str, Query> {
    alt((add, done, search))(input)
}

fn add(input: &str) -> IResult<&str, Query> {
    match preceded(
        pair(tag("add"), space1),
        pair(description, preceded(space0, tags)),
    )(input)
    {
        Err(e) => Err(e),
        Ok((rest, (d, ts))) => Ok((rest, Query::Add(Description::new(&d), ts))),
    }
}

fn is_lowercase_or_dash_or_whitespace(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_whitespace() || c == '-'
}

fn is_lowercase_or_dash(c: char) -> bool {
    c.is_ascii_lowercase() || c == '-'
}

fn sentence(input: &str) -> IResult<&str, &str> {
    take_while(is_lowercase_or_dash_or_whitespace)(input)
}

fn word(input: &str) -> IResult<&str, &str> {
    take_while1(is_lowercase_or_dash)(input)
}

fn raw_word(input: &str) -> IResult<&str, String> {
    match word(input) {
        Err(e) => Err(e),
        Ok((rest, w)) => Ok((rest, w.to_string())),
    }
}

fn todo_tag(input: &str) -> IResult<&str, &str> {
    preceded(tag("#"), word)(input)
}

fn raw_tag(input: &str) -> IResult<&str, String> {
    match pair(tag("#"), word)(input) {
        Err(e) => Err(e),
        Ok((rest, (h, w))) => Ok((rest, format!("{}{}", h, w))),
    }
}

fn description(input: &str) -> IResult<&str, String> {
    match delimited(tag("\""), sentence, tag("\""))(input) {
        Err(e) => Err(e),
        Ok((rest, d)) => Ok((rest, d.to_string())),
    }
}

fn tags(input: &str) -> IResult<&str, Vec<Tag>> {
    match separated_list(space1, todo_tag)(input) {
        Err(e) => Err(e),
        Ok((rest, ts)) => Ok((rest, ts.iter().map(|w| Tag::new(w)).collect())),
    }
}

fn done(input: &str) -> IResult<&str, Query> {
    match preceded(pair(tag("done"), space1), many1(digit1))(input) {
        Err(e) => Err(e),
        Ok((rest, i)) => Ok((rest, Query::Done(Index::new(vec_to_u64(i))))),
    }
}

fn vec_to_u64(dss: Vec<&str>) -> u64 {
    let ds = dss
        .iter()
        .fold("".to_string(), |acc, x| format!("{}{}", acc, x));
    ds.parse::<u64>().unwrap()
}

enum SearchWordOrTag {
    RawWord(String),
    RawTag(String),
}

fn search(input: &str) -> IResult<&str, Query> {
    match preceded(
        tag("search"),
        opt(preceded(space1, separated_list(space1, search_word_or_tag))),
    )(input)
    {
        Err(e) => Err(e),
        Ok((rest, mash)) => Ok((rest, mash_to_query(mash.unwrap_or_default()))),
    }
}

fn search_word_or_tag(input: &str) -> IResult<&str, SearchWordOrTag> {
    match alt((raw_tag, raw_word))(input) {
        Err(e) => Err(e),
        Ok((rest, wot)) => Ok((
            rest,
            if let Some(wot) = wot.strip_prefix('#') {
                SearchWordOrTag::RawTag(wot.to_string())
            } else {
                SearchWordOrTag::RawWord(wot.to_string())
            },
        )),
    }
}

fn mash_to_query(mash: Vec<SearchWordOrTag>) -> Query {
    let mut search_words: Vec<SearchWord> = vec![];
    let mut tags: Vec<Tag> = vec![];
    for i in mash {
        match i {
            SearchWordOrTag::RawWord(w) => search_words.push(SearchWord::new(&w)),
            SearchWordOrTag::RawTag(t) => tags.push(Tag::new(&t)),
        }
    }
    Query::Search(SearchParams {
        words: search_words,
        tags,
    })
}
