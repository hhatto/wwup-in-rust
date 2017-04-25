//#![feature(process_abort)]
use std::process;

extern "C" fn _atexit() {
    println!("at exit");
}

fn main() {
    process::exit(22);
    // process::abort();
}
