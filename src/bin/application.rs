extern crate todo_swamp; //I'm on Ubuntu now instead of Windows!

use todo_swamp::*;
use std::io;

pub fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let file_name = &args[1];
    file_run(file_name)

    //standard_run()
}
