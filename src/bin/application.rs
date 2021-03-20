extern crate todo_swamp;

use std::io::{self, prelude::*, Write, BufReader, BufWriter};
use std::fs::File;
use std::env;

use todo_swamp::*;

const BENCHMARK_SUFFIX: [&str; 5] = ["","_2","_3","_4","_5"];

pub fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();
    // let file_name = &args[1];
    standard_run()
}

// fn benchmark(file_name: &str) -> io::Result<f64> {
//     for suffix in BENCHMARK_SUFFIX.iter() {
//         let mut file_name = file_name.to_owned();
//         file_name.push_str(suffix);
//         file_run(&file_name)?;
//     }
//     Ok(4.0)
// }

fn file_run(file_name: &str) -> io::Result<()> {
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
        buffer_out.flush()?;
    }
    Ok(())
}
