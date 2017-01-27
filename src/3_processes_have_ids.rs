extern crate nix;

use std::time::Duration;
use std::thread;
use nix::unistd;


fn main() {
    let pid = unistd::getpid();
    println!("this process's PID is {}", pid);
    thread::sleep(Duration::from_millis(10_000));
}
