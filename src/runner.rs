use crate::*;

pub fn run_line(line: &str, tl: &mut TodoList) {
    if let Ok((_, q)) = parser::query(line) {
        match run_query(q, tl) {
            Ok(r) => { println!("{}", r); },
            Err(e) => { eprintln!("Error: {}", e); },
        }
    }
}

fn run_query(q: Query, tl: &mut TodoList) -> Result<QueryResult, QueryError> {
    match q {
        Query::Add(desc, tags) => {
            unimplemented!()
        },
        Query::Done(idx) => {
            unimplemented!()
        },
        Query::Search(params) => {
            unimplemented!()
        },
    }
}
