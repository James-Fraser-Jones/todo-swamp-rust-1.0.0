use crate::*;

pub fn run_line(line: &str, tl: &mut TodoList) {
    if let Ok((_, q)) = parser::query(line) {
        match run_query(q, tl) {
            Ok(r) => { println!("{}", r); },
            Err(e) => { eprintln!("Error: {}", e); },
        }
    }
    else { //TODO: figure out whether this is really necessary
        panic!("Failed to parse command: {:?}", line);
    }
}

fn run_query(q: Query, tl: &mut TodoList) -> Result<QueryResult, QueryError> {
    match q {
        Query::Add(desc, tags) => {
            let item = tl.push(desc, tags);
            Ok(query::QueryResult::Added(item))
        },
        Query::Done(idx) => {
            match tl.done_with_index(idx) {
                Some(_) => Ok(query::QueryResult::Done),
                None => Err(QueryError(String::from("Attempted to mark non-existent item as done"))), //TODO: figure out whether this is the right thing to do
            }
        },
        Query::Search(params) => {
            let results = tl.search(params);
            let results = results.into_iter().map(|r| r.clone()).collect(); //TODO: figure out whether this is inefficient
            Ok(query::QueryResult::Found(results))
        },
    }
}
