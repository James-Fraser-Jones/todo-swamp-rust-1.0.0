use crate::*;

pub fn run_line<T: TodoLister>(line: &str, tl: &mut T) -> Option<QueryResult> {
    match parser::query(line) {
        Ok((_, q)) => match run_query(q, tl) {
            Ok(r) => Some(r),
            Err(e) => { 
                eprintln!("Error: {}", e);
                None
            },
        }
        Err(e) => {
            eprintln!("Error: {}", e); 
            eprintln!("Attempted to parse: \"{}\"", line);
            None
        }, 
    }
}

fn run_query<T: TodoLister>(q: Query, tl: &mut T) -> Result<QueryResult, QueryError> {
    match q {
        Query::Add(desc, tags) => {
            let item = tl.push(desc, tags);
            Ok(query::QueryResult::Added(item))
        },
        Query::Done(idx) => {
            match tl.done_with_index(idx) {
                Some(_) => Ok(query::QueryResult::Done),
                None => Err(QueryError(String::from("Attempted to mark non-existent item as Done"))),
            }
        },
        Query::Search(params) => {
            let results = tl.search(params);
            let results = results.into_iter().map(|r| r.clone()).collect();
            Ok(query::QueryResult::Found(results))
        },
    }
}
