pub mod parser;
pub mod query;
pub mod runner;
pub mod todo_list;
pub mod trie;

pub use todo_list::*;
pub use query::*;
pub use trie::*;

use std::io::{self, prelude::*};
use std::fs;

#[inline]
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

#[inline]
pub fn file_run(file_name: &str) -> io::Result<()> {
    let file_in = fs::File::open(format!("{}.in", file_name))?;
    let file_out = fs::File::create(format!("{}.out", file_name))?;

    let mut lines_in = io::BufReader::new(file_in).lines();
    let mut buffer_out = io::BufWriter::new(file_out);

    let mut tl: TriedoList<Trie4> = TriedoList::new();

    if let Some(Ok(_s)) = lines_in.next() {
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

/* Command to test resulting files (0 means files are equal)
cmp tests/Ak/benchmark_Ak_N_test.out tests/Ak/benchmark_Ak_N_testB.out ; echo $?
*/
pub fn file_test(file_name: &str) -> io::Result<()> {
    file_test_single(file_name, "", TodoList::new())?;
    //file_test_single(file_name, "1", TriedoList::<Trie1>::new())?; //this one is too slow to be used for testing
    file_test_single(file_name, "2", TriedoList::<Trie2>::new())?;
    file_test_single(file_name, "3", TriedoList::<Trie3>::new())?;
    file_test_single(file_name, "4", TriedoList::<Trie4>::new())
}
fn file_test_single<T: TodoLister>(file_name: &str, out: &str, mut tl: T) -> io::Result<()> {
    let file_in = fs::File::open(format!("{}.in", file_name))?;
    let file_out = fs::File::create(format!("{}_test{}.out", file_name, out))?;
    let mut lines_in = io::BufReader::new(file_in).lines();
    let mut buffer_out = io::BufWriter::new(file_out);
    if let Some(Ok(_s)) = lines_in.next() {
        for line in lines_in {
            if let Ok(l) = line {
                if let Some(mut r) = runner::run_line(&l, &mut tl) {
                    if let QueryResult::Found(results) = &mut r { //sorted results makes resulting test files easy to check for equality
                        results.sort();
                    }
                    writeln!(buffer_out, "{}", r)?;
                }
                else {
                    panic!();
                }
            }
        }
    }
    Ok(())
}
