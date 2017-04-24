#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate nix;
extern crate libc;

use std::time::Duration;
use std::thread;
use chan_signal::Signal;
use nix::unistd;
use nix::sys::wait;
use nix::sys::signal::{kill, sigaction, SigAction, SigHandler, SIGINT, SIGCHLD, SA_RESETHAND, SigSet};

extern fn handle_sigint(i: i32) {
    println!("handler. i={}", i);
}

fn main() {
    let sa = SigAction::new(SigHandler::Handler(handle_sigint),
                                          SA_RESETHAND,
                                          SigSet::empty());
    unsafe { sigaction(SIGCHLD, &sa) }.unwrap();

    let pid = unistd::getpid();
    let child_processes = 3;
    let mut dead_processes = 0;
    println!("pid={}", pid);

    let signal = chan_signal::notify(&[Signal::CHLD, Signal::INT]);
    thread::spawn(move || signal_handler(signal));

    for _ in 0..child_processes {
        match unistd::fork().expect("fork() error") {
            unistd::ForkResult::Parent{ child } => {
                println!("fork prent proc. child={}", child);
            }
            unistd::ForkResult::Child => {
                let child_pid = unistd::getpid();
                println!("fork child proc. pid={}", child_pid);
                unistd::sleep(3);
                return;
            }
        }
    }

    let _ = kill(pid, SIGINT);

    loop {
        let wait_status = wait::waitpid(-1, Some(wait::WNOHANG));
        match wait_status {
            Ok(wait::WaitStatus::Exited(pid_t, _)) => {
                println!("exit={}", pid_t);
                dead_processes += 1;
            },
            Ok(_) => {
                println!("other status.");
                thread::sleep(Duration::from_millis(200));
            },
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
        if dead_processes == child_processes {
            println!("all child procs are finished");
            break;
        }
    }
}

fn signal_handler(signal: chan::Receiver<Signal>) {
    loop {
        chan_select! {
            signal.recv() -> signal => {
                println!("received signal: {:?}", signal)
            },
        }
    }
}
