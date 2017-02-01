extern crate libc;

use std::env;


fn main() {
    for arg in env::args() {
        if arg == "--help" {
            println!("usage: arg");
            break;
        }
    }
}
