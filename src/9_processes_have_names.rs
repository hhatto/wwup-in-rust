extern crate prctl;

#[cfg(target_os="linux")]
use std::thread;
#[cfg(target_os="linux")]
use std::time::Duration;

#[cfg(target_os="linux")]
fn _main() {
    let _ = prctl::set_name("wwupproc");
    thread::sleep(Duration::from_millis(10_000));   // show with top command!!
}

#[cfg(not(target_os="linux"))]
fn _main() {}

fn main() {
    _main();
}
