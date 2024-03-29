extern crate nix;

use std::process;
use nix::unistd;

fn main() {
    let pid = unistd::getpid();
    println!("pid={}", pid);

    match unsafe{unistd::fork().expect("fork() error")} {
        unistd::ForkResult::Parent { .. } => {
            process::exit(0);
        }
        unistd::ForkResult::Child => {
            for _ in 0..5 {
                unistd::sleep(1);
                println!("I'm an orphan!");
            }
        }
    }
}
