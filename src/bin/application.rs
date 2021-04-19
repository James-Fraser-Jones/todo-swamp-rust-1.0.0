extern crate todo_swamp;

use todo_swamp::*;

use std::io::{self, prelude::*};
use std::fs;
use std::time;

pub fn main() -> io::Result<()> {
    // let args: Vec<String> = std::env::args().collect();
    // let file_name = &args[1];

    // //run_count
    // for i in 1..=A {
    //     benchmark_run_count(&format!("tests/test{}", i), B::new(), C)?;
    //     println!("Done: {}", i);
    // }

    // //run_timed
    // for i in 1..=A {
    //     let commands_processed = benchmark_run_timed(&format!("tests/test{}", i), B::new(), C)?;
    //     println!("{}", commands_processed);
    // }

    for i in 1..=5 {
        let commands_processed = benchmark_run_timed(&format!("tests/test{}", i), TodoList2::new(), 10000)?;
        println!("{}", commands_processed);
    }

    Ok(())
}

fn _standard_run<T: TodoLister>(mut tl: T) -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut lines_in = stdin.lock().lines();
    let mut buffer_out = stdout.lock();
    if let Some(Ok(_s)) = lines_in.next() { //read first line as query count, loop on remaining lines
        for line in lines_in {
            if let Ok(l) = line {
                if let Some(r) = runner::run_line(&l, &mut tl) {
                    writeln!(buffer_out, "{}", r)?;
                }
            }
        }
        buffer_out.flush()?; //TODO: figure out whether we need to flush the others
    }
    Ok(())
}

fn _file_run<T: TodoLister>(file_name: &str, append: &str, mut tl: T) -> io::Result<()> {
    let file_in = fs::File::open(format!("{}.in", file_name))?;
    let file_out = fs::File::create(format!("{}{}.out", file_name, append))?;
    let mut lines_in = io::BufReader::new(file_in).lines();
    let mut buffer_out = io::BufWriter::new(file_out);
    if let Some(Ok(_s)) = lines_in.next() {
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

//returns number of responses (multi-line search results count as a single response)
fn benchmark_run_timed<T: TodoLister>(file_name: &str, mut tl: T, max_millis: u128) -> io::Result<usize> {
    let file_in = fs::File::open(format!("{}.in", file_name))?;
    let mut lines_in = io::BufReader::new(file_in).lines();
    let mut count = 0;
    if let Some(Ok(_s)) = lines_in.next() {
        let start = time::Instant::now();
        for line in lines_in {
            if start.elapsed().as_millis() > max_millis {
                break
            }
            if let Ok(l) = line {
                if let Some(result) = runner::run_line(&l, &mut tl) {
                    black_box(result);
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}

fn _benchmark_run_count<T: TodoLister>(file_name: &str, mut tl: T, num_commands: usize) -> io::Result<()> {
    let file_in = fs::File::open(format!("{}.in", file_name))?;
    let mut lines_in = io::BufReader::new(file_in).lines();
    let mut count = 0;
    if let Some(Ok(_s)) = lines_in.next() {
        for line in lines_in {
            if count >= num_commands {
                break
            }
            if let Ok(l) = line {
                if let Some(result) = runner::run_line(&l, &mut tl) {
                    black_box(result);
                    count += 1;
                }
            }
        }
    }
    Ok(())
}

fn _test_run<T: TodoLister>(file_name: &str, append: &str, mut tl: T, num_lines: usize) -> io::Result<()> {
    let file_in = fs::File::open(format!("{}.in", file_name))?;
    let file_out = fs::File::create(format!("{}{}.out", file_name, append))?;
    let mut lines_in = io::BufReader::new(file_in).lines();
    let mut buffer_out = io::BufWriter::new(file_out);
    let mut count = 0;
    if let Some(Ok(_s)) = lines_in.next() {
        for line in lines_in {
            if count >= num_lines {
                break
            }
            if let Ok(l) = line {
                if let Some(mut r) = runner::run_line(&l, &mut tl) {
                    if let QueryResult::Found(results) = &mut r { 
                        results.sort();             //sorted results makes resulting test files easy to check for equality
                        count += results.len() - 1; //search results can produce more than one line, count needs to be updated accordingly
                    }
                    writeln!(buffer_out, "{}", r)?;
                    count += 1;
                }
                else { 
                    panic!(); //make bugs more apparent
                }
            }
        }
    }
    Ok(())
}

//copied from criterion: https://docs.rs/criterion/0.3.4/src/criterion/lib.rs.html#174-180
pub fn black_box<T>(dummy: T) -> T { 
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}