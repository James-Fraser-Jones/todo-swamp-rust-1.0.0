extern crate todo_swamp;

use todo_swamp::*;

use std::io::{self, prelude::*};
use std::fs;
use std::time;

pub fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let file_name = &args[1];

    test_run(file_name, "_output", TriedoList::<Trie4>::new(), Some(10000), false)
}

pub fn standard_run() -> io::Result<()> {
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

/* Command to test resulting files when sort == true (0 means files are equal)
cmp tests/Ak/benchmark_Ak_N_test.out tests/Ak/benchmark_Ak_N_testB.out ; echo $?
*/
pub fn test_run<T: TodoLister>(file_name: &str, append: &str, mut tl: T, millis: Option<u128>, sort: bool) -> io::Result<()> {
    let file_in = fs::File::open(format!("{}.in", file_name))?;
    let file_out = fs::File::create(format!("{}{}.out", file_name, append))?;
    let mut lines_in = io::BufReader::new(file_in).lines();
    let mut buffer_out = io::BufWriter::new(file_out);
    if let Some(Ok(_s)) = lines_in.next() {
        let start = time::Instant::now();
        for line in lines_in {
            if let Some(millis) = millis {
                if start.elapsed().as_millis() > millis {
                    writeln!(buffer_out, "TASK FORCED TO EXIT AFTER: {}MS", millis)?;
                    break
                }
            }
            if let Ok(l) = line {
                if let Some(mut r) = runner::run_line(&l, &mut tl) {
                    if sort {
                        if let QueryResult::Found(results) = &mut r { 
                            results.sort(); //sorted results makes resulting test files easy to check for equality
                        }
                    }
                    writeln!(buffer_out, "{}", r)?;
                }
                else { 
                    panic!(); //make bugs more apparent
                }
            }
        }
    }
    Ok(())
}