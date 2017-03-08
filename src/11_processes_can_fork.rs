extern crate nix;

use nix::unistd;

fn main() {
    let pid = unistd::getpid();
    println!("pid={}", pid);

    match unistd::fork().expect("fork() error") {
        unistd::ForkResult::Parent{ child } => {
            let pid = unistd::getpid();
            println!("parent. child={}, pid={}", child, pid);
            unistd::sleep(1);
        }
        unistd::ForkResult::Child => {
            let pid = unistd::getpid();
            println!("child. pid={}", pid);
        }
    }
}
