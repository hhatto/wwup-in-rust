extern crate nix;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::os::unix::io::FromRawFd;
use std::os::unix::net::UnixStream;
use nix::unistd;

fn example_unix_socket_pair() {
    let (mut child_sock, mut parent_sock) = UnixStream::pair().expect("pair() error");

    match unsafe{unistd::fork().expect("fork() error")} {
        unistd::ForkResult::Parent { child } => {
            println!("child={}", child);
        }
        unistd::ForkResult::Child => {
            let child_pid = unistd::getpid();
            println!("I'm an orphan! {}", child_pid);

            drop(parent_sock);

            let rr = child_sock.try_clone().expect("");
            let mut line_reader = BufReader::new(rr);
            let mut line = String::new();
            let len = line_reader.read_line(&mut line).expect("read_line() error");
            print!("len={}, parent -> child msg={}", len, line);

            for i in 0..5 {
                match child_sock.write_all(format!("child-proc. seq:{} with unix socket\n", i).as_bytes()) {
                    Ok(_) => {}
                    Err(e) => println!("write_all() error. e={:?}", e),
                }
                unistd::sleep(1);
            }
            ::std::process::exit(0);
        }
    }

    drop(child_sock);

    match parent_sock.write_all(b"hoge\n") {
        Ok(_) => {}
        Err(e) => println!("write_all() error. e={:?}", e),
    }

    let rr = parent_sock.try_clone().expect("");
    let mut line_reader = BufReader::new(rr);
    let rrr = parent_sock.try_clone().expect("");
    let mut line_writer = ::std::io::BufWriter::new(rrr);
    loop {
        let mut line = String::new();
        let len = line_reader.read_line(&mut line).expect("read_line() error");
        if len <= 0 {
            break;
        }
        match line_writer.write_all(b"hello\n") {
            Ok(_) => {}
            Err(e) => println!("error. e={:?}", e),
        }
        print!("msg={}", line);
    }
}

fn example_pipe() {
    let (reader, writer) = nix::unistd::pipe().expect("pipe() error");
    let r = unsafe { File::from_raw_fd(reader) };
    let mut w = unsafe { File::from_raw_fd(writer) };
    match unsafe{unistd::fork().expect("fork() error")} {
        unistd::ForkResult::Parent { child } => {
            println!("child={}", child);
        }
        unistd::ForkResult::Child => {
            let child_pid = unistd::getpid();
            println!("I'm an orphan! {}", child_pid);
            drop(r);
            for i in 0..5 {
                // error occured when write to reader
                // match r.write_all(format!("child-proc. seq:{}\n", i).as_bytes()) {
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
