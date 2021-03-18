extern crate todo_swamp;

use std::io::{self, prelude::*, Write, BufReader, BufWriter};
use std::fs::File;
use std::env;

use todo_swamp::*;

pub fn main() -> io::Result<()> {
    //standard_run()
    file_run()
}

fn file_run() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_in = File::open(format!("{}.in", file_name))?;
    let file_out = File::create(format!("{}.out", file_name))?;

    let mut lines_in = BufReader::new(file_in).lines();
    let mut buffer_out = BufWriter::new(file_out);

    let mut tl: TodoList = TodoList::new();
    if let Some(Ok(_s)) = lines_in.next() { //read first line as query count, loop on remaining lines
        for line in lines_in {
            if let Ok(l) = line {
                if let Some(r) = runner::run_line(&l, &mut tl) {
                    writeln!(buffer_out, "{}", r)?;
                }
                else { //make bugs more apparent
                    panic!();
                }
            }
        }
    }
    Ok(())
}

fn standard_run() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut lines_in = stdin.lock().lines();
    let mut buffer_out = stdout.lock();

    let mut tl: TodoList = TodoList::new();
    if let Some(Ok(_s)) = lines_in.next() { //read first line as query count, loop on remaining lines
        for line in lines_in {
            if let Ok(l) = line {
                if let Some(r) = runner::run_line(&l, &mut tl) {
                    writeln!(buffer_out, "{}", r)?;
                }
            }
        }
    }
    Ok(())
}
