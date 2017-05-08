extern crate nix;

use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::process;
use nix::unistd;
use nix::sys::ioctl::libc::{getpgrp, setsid, getsid, STDIN_FILENO, STDOUT_FILENO, STDERR_FILENO};

fn daemonize() {
    let pgrpid = unsafe { getpgrp() };
    let sid = unsafe { getsid(0) };
    let pid = unistd::getpid();
    println!("pgrp={}, sid={}, pid={}", pgrpid, sid, pid);

    match unistd::fork().expect("fork() error") {
        unistd::ForkResult::Parent { child } => {
            let pgrpid = unsafe { getpgrp() };
            let sid = unsafe { getsid(0) };
            println!("pgrp={}, sid={}, child={}", pgrpid, sid, child);
            process::exit(0);
        }
        unistd::ForkResult::Child => {
            let child_pid = unistd::getpid();
            println!("I'm an child! {}", child_pid);
        }
    }

    // through only child process
    let newsid = unsafe { setsid() };
    let newgetsid = unsafe { getsid(0) };
    println!("setsid... pgrp={}, newsid={}, newgetsid={}, sid={}, pid={}",
             pgrpid, newsid, newgetsid, sid, pid);

    match unistd::fork().expect("fork() error") {
        unistd::ForkResult::Child => {
            let child_pid = unistd::getpid();
            println!("I'm an child! {}", child_pid);
        }
        _ => { process::exit(0); }
    }

    unistd::chdir("/").expect("chdir() error");

    // standard steams to go to devnull
    let devnull = File::open("/dev/null").expect("/dev/null is not open");
    unistd::close(STDIN_FILENO).expect("close(stdin) error");
    unistd::dup2(devnull.as_raw_fd(), STDIN_FILENO).expect("dup2(stdin) error");
    unistd::close(STDOUT_FILENO).expect("unistd::close(stdout) error");
    unistd::dup2(devnull.as_raw_fd(), STDOUT_FILENO).expect("dup2(stdout) error");
    unistd::close(STDERR_FILENO).expect("unistd::close(stderr) error");
    unistd::dup2(devnull.as_raw_fd(), STDERR_FILENO).expect("dup2(stderr) error");

    unistd::sleep(10);
}

fn main() {
    daemonize();
}
