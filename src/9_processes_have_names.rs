extern crate prctl;

use std::thread;
use std::time::Duration;

fn main() {
    let _ = prctl::set_name("wwupproc");
    thread::sleep(Duration::from_millis(10_000));   // show with top command!!
}
