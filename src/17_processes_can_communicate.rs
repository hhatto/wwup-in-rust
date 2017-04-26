extern crate nix;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::os::unix::io::FromRawFd;
use std::os::unix::net::UnixStream;
use nix::unistd;

fn example_unix_socket_pair() {
    let (rsock, mut wsock) = match UnixStream::pair() {
        Ok((rsock, wsock)) => (rsock, wsock),
        Err(e) => {
            println!("Couldn't create a pair of sockets: {:?}", e);
            return
        }
    };

    match unistd::fork().expect("fork() error") {
        unistd::ForkResult::Parent { child } => {
            println!("child={}", child);
        }
        unistd::ForkResult::Child => {
            let child_pid = unistd::getpid();
            println!("I'm an orphan! {}", child_pid);
            drop(rsock);
            for i in 0..5 {
                match wsock.write_all(format!("child-proc. seq:{} with unix socket\n", i).as_bytes()) {
                    Ok(_) => {}
                    Err(e) => println!("write_all() error. e={:?}", e),
                }
                unistd::sleep(1);
            }
            ::std::process::exit(0);
        }
    }
    drop(wsock);
    let mut line_reader = BufReader::new(rsock);
    loop {
        let mut line = String::new();
        let len = line_reader.read_line(&mut line).expect("read_line() error");
        if len <= 0 {
            break;
        }
        print!("msg={}", line);
    }
}

fn example_pipe() {
    let (reader, writer) = nix::unistd::pipe().expect("pipe() error");
    let r = unsafe { File::from_raw_fd(reader) };
    let mut w = unsafe { File::from_raw_fd(writer) };
    match unistd::fork().expect("fork() error") {
        unistd::ForkResult::Parent { child } => {
            println!("child={}", child);
        }
        unistd::ForkResult::Child => {
            let child_pid = unistd::getpid();
            println!("I'm an orphan! {}", child_pid);
            drop(r);
            for i in 0..5 {
                // error occured when write to reader
                //match r.write_all(format!("child-proc. seq:{}\n", i).as_bytes()) {
                match w.write_all(format!("child-proc. seq:{} with unix pipe\n", i).as_bytes()) {
                    Ok(_) => {}
                    Err(e) => println!("write_all() error. e={:?}", e),
                }
                unistd::sleep(1);
            }
            ::std::process::exit(0);
        }
    }
    drop(w);
    let mut line_reader = BufReader::new(r);
    loop {
        let mut line = String::new();
        let len = line_reader.read_line(&mut line).expect("read_line() error");
        if len <= 0 {
            break;
        }
        print!("msg={}", line);
    }
}

fn main() {
    example_pipe();
    example_unix_socket_pair();
}
