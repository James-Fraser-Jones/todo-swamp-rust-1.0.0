extern crate todo_swamp;

use todo_swamp::*;

use std::io::{self, prelude::*};
use std::fs;
use std::time;

pub fn main() -> io::Result<()> {
    standard_run(TodoList::new())
}

//runs the program, taking input from the standard input and outputs to the standard output
#[allow(dead_code)]
fn standard_run<T: TodoLister>(mut tl: T) -> io::Result<()> {
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
    }
    Ok(())
}

//takes input from the specified file 
//outputs to an output file
#[allow(dead_code)]
fn file_run<T: TodoLister>(file_name: &str, append: &str, mut tl: T) -> io::Result<()> {
    let file_in = fs::File::open(format!("{}.in", file_name))?;
    let file_out = fs::File::create(format!("{}_{}.out", file_name, append))?;
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

//takes input from the specified file
//outputs to an output file 
//returns the number of queries it was able to respond to, in the specified number of milliseconds
#[allow(dead_code)]
fn file_run_timed<T: TodoLister>(file_name: &str, append: &str, mut tl: T, max_millis: u128) -> io::Result<usize> {
    let file_in = fs::File::open(format!("{}.in", file_name))?;
    let file_out = fs::File::create(format!("{}_{}.out", file_name, append))?;
    let mut lines_in = io::BufReader::new(file_in).lines();
    let mut buffer_out = io::BufWriter::new(file_out);
    let mut count = 0;
    if let Some(Ok(_s)) = lines_in.next() {
        let start = time::Instant::now();
        for line in lines_in {
            if start.elapsed().as_millis() > max_millis {
                break
            }
            if let Ok(l) = line {
                if let Some(r) = runner::run_line(&l, &mut tl) {
                    writeln!(buffer_out, "{}", r)?;
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}

//takes input from the specified file 
//returns the number of commands it was able to respond to, in the specified number of milliseconds
#[allow(dead_code)]
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

//takes input from the specified file
//takes as much time as it needs, to process the specified number of commands
#[allow(dead_code)]
fn benchmark_run_count<T: TodoLister>(file_name: &str, mut tl: T, num_commands: usize) -> io::Result<()> {
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

//runs correctness_run on all current implementations to stress test for implementation incorrectness
#[allow(dead_code)]
fn correctness_all(test: &str, num_commands: usize) -> io::Result<()> {
    correctness_run("tests", test, "naive", TodoList::new(), num_commands)?;
    correctness_run("tests", test, "naive2", TodoList2::new(), num_commands)?;
    correctness_run("tests", test, "trie1", TriedoList::<Trie1>::new(), num_commands)?;
    correctness_run("tests", test, "trie2", TriedoList::<Trie2>::new(), num_commands)?;
    correctness_run("tests", test, "trie3", TriedoList::<Trie3>::new(), num_commands)?;
    correctness_run("tests", test, "trie4", TriedoList::<Trie4>::new(), num_commands)?;
    Ok(())
}

//takes input from the specified file
//takes as much time as it needs, to process the specified number of commands
//outputs to an output file (with search query results sorted so that outputs from different implementations can be easily compared for equality)
#[allow(dead_code)]
fn correctness_run<T: TodoLister>(dir: &str, name: &str, append: &str, mut tl: T, num_commands: usize) -> io::Result<()> {
    let file_in = fs::File::open(format!("{}/{}.in", dir, name))?;
    fs::create_dir_all(format!("{}/correct", dir))?;
    let file_out = fs::File::create(format!("{}/correct/{}_{}.out", dir, name, append))?;
    let mut lines_in = io::BufReader::new(file_in).lines();
    let mut buffer_out = io::BufWriter::new(file_out);
    let mut count = 0;
    if let Some(Ok(_s)) = lines_in.next() {
        for line in lines_in {
            if count >= num_commands {
                break
            }
            if let Ok(l) = line {
                if let Some(mut r) = runner::run_line(&l, &mut tl) {
                    if let QueryResult::Found(results) = &mut r { 
                        results.sort(); //sorted results makes resulting test files easy to check for equality
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