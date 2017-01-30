extern crate nix;
extern crate libc;

use std::fs::File;
use std::os::unix::io::AsRawFd;
use libc::{STDIN_FILENO, STDOUT_FILENO, STDERR_FILENO};

fn main() {
    let passwd = File::open("/etc/passwd").expect("file open error");
    println!("{}", passwd.as_raw_fd());

    let hosts = File::open("/etc/hosts").expect("file open error");
    println!("{}", hosts.as_raw_fd());

    drop(passwd);

    let null = File::open("/dev/null").expect("/dev/null is not open");
    println!("{}", null.as_raw_fd());

    println!("{}, {}, {}", STDIN_FILENO, STDOUT_FILENO, STDERR_FILENO);
}
