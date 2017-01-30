extern crate nix;

use std::time::Duration;
use std::thread;
use nix::unistd;


fn main() {
    let pid = unistd::getpid();
    let ppid = unistd::getppid();
    println!("this process's PID is {}", pid);
    println!("this process's parent PID is {}", ppid);
    thread::sleep(Duration::from_millis(10_000));
}
