extern crate todo_swamp;

use std::io::{self, prelude::*, BufReader};
use std::fs::File;
use std::env;

use todo_swamp::*;

pub fn main() {

    //use "trait objects" to do dynamic dispatch for getting buffered reader for either standard input or a specified file
    let stdin = io::stdin();
    let mut args = env::args();
    let buf_reader: Box<dyn io::BufRead>;
    if let Some(file_name) = args.nth(1) {
        let file = File::open(&file_name).expect(
            &format!("Error failed to open file: \"{}\"", file_name)
        );
        buf_reader = Box::new(BufReader::new(file)); 
    }
    else {
        buf_reader = Box::new(stdin.lock());
    }
    
    //read first line as query count, loop on remaining lines
    let mut tl: TodoList = TodoList::new();
    let mut lines = buf_reader.lines();
    if let Some(Ok(_s)) = lines.next() {
        // let query_count: i32 = s.trim().parse().expect(
        //     &format!("Error parsing integer from first line of input: \"{}\"", s)
        // );
        for line in lines {
            if let Ok(l) = line {
                runner::run_line(&l, &mut tl);
            }
        }
    }
}
