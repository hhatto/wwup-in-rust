extern crate nix;

use std::process;
use nix::unistd;
use nix::sys::wait;

fn main() {
    let message = "Good Morning";
    let recipient = "tree@mybackyard.com";
    let pid = unistd::getpid();
    println!("pid={}", pid);

    match unistd::fork().expect("fork() error") {
        unistd::ForkResult::Parent { child } => {
            println!("child={}, {}, {}", child, message, recipient);

            //match wait::wait().expect("wait() error") {
            //    wait::WaitStatus::Exited(pid_t, _) => println!("{}", pid_t),
            //    _ => println!("child proc still alive"),
            //}

            match wait::waitpid(child, None).expect("wait() error") {
                wait::WaitStatus::Exited(pid_t, _) => println!("{}", pid_t),
                _ => println!("child proc still alive"),
            }
            process::exit(0);
        }
        unistd::ForkResult::Child => {
            let child_pid = unistd::getpid();
            for _ in 0..5 {
                unistd::sleep(1);
                println!("I'm an orphan! {}", child_pid);
            }
        }
    }
}
