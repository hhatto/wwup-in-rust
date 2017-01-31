extern crate libc;

use std::env;
use std::process::Command;


fn main() {
    println!("{}", env::var("MESSAGE").unwrap_or("envvar not found".to_string()));
    env::set_var("MESSAGE", "wing it");

    // not expanded environment variable
    let output = Command::new("echo").arg("\"${MESSAGE}\"").output().expect("command execution fail");
    let out = output.stdout;
    let err = output.stderr;
    println!("stdout={}, stderr={}", String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap());

    println!("{}", env::var("MESSAGE").unwrap());
}
