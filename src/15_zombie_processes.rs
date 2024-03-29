extern crate nix;

use nix::unistd;

fn main() {
    let message = "Good Morning";
    let recipient = "tree@mybackyard.com";
    let pid = unistd::getpid();
    println!("pid={}", pid);

    match unsafe{unistd::fork().expect("fork() error")} {
        unistd::ForkResult::Parent { child } => {
            println!("child={}, {}, {}", child, message, recipient);
            for _ in 0..10 {
                unistd::sleep(1);
            }
        }
        unistd::ForkResult::Child => {
            let child_pid = unistd::getpid();
            println!("I'm an orphan! {}", child_pid);
            unistd::sleep(1);
        }
    }
}
