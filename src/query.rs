use std::fmt;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Query {
    Add (Vec<Word>, Vec<Tag>),
    Done (Index),
    Search (SearchParams),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchParams {
    pub params : Vec<todo_list::SearchWordOrTag>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryResult {
    Added (TodoItem),
    Done,
    Found (Vec<todo_list::TodoItem>),
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            QueryResult::Added(ti) => write!(f, "{}", ti.index),
            QueryResult::Done => write!(f, "done"),
            QueryResult::Found(rs) => {
                let mut buff : Vec<String> = vec![];
                buff.push(format!("{} item(s) found", rs.len()));
                for i in rs {
                    buff.push(format!("{}", i));
                }
                write!(f, "{}", buff.join("\n"))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryError(pub String);

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred while processing the query: {}.", self.0)
    }
}
