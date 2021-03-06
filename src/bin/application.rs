extern crate todo_swamp;

use std::io;
use std::io::prelude::*;

use todo_swamp::*;

pub fn main() {
    let mut tl: TodoList = TodoList::new();

    //get lines as iterator from stdin
    let stdin = io::stdin(); 
    let mut lines = stdin.lock().lines();

    if let Some(Ok(_s)) = lines.next() {
        // let line_num: i32 = s.trim().parse().expect(
        //     &format!("Error parsing integer from first line of input: \"{}\"", s)[..]
        // );
        for line in lines {
            if let Ok(l) = line {
                runner::run_line(&l, &mut tl);
            }
        }
    }
}
