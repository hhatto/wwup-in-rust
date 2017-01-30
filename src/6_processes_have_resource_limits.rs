extern crate libc;

use std::io::prelude::Write;
use std::fs::OpenOptions;
use libc::rlimit;


fn main() {
    let mut rlimit = rlimit{rlim_cur: 0, rlim_max: 0};
    unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut rlimit); }
    println!("{}, {}", rlimit.rlim_cur, rlimit.rlim_max);

    rlimit.rlim_cur = rlimit.rlim_cur * 2;
    unsafe { libc::setrlimit(libc::RLIMIT_NOFILE, &mut rlimit); }

    let mut rlimit = rlimit{rlim_cur: 0, rlim_max: 0};
    unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut rlimit); }
    println!("{}, {}", rlimit.rlim_cur, rlimit.rlim_max);

    rlimit.rlim_cur = 3;
    unsafe { libc::setrlimit(libc::RLIMIT_NOFILE, &mut rlimit); }

    let null = OpenOptions::new().write(true).open("/dev/null");
    match null {
        Ok(mut f) => { let _ = f.write(b"test"); },
        Err(e) => { println!("{}", e); },
    }

    let mut rlimit = rlimit{rlim_cur: 0, rlim_max: 0};
    unsafe { libc::getrlimit(libc::RLIMIT_NPROC, &mut rlimit); }
    println!("{}, {}", rlimit.rlim_cur, rlimit.rlim_max);

    let mut rlimit = rlimit{rlim_cur: 0, rlim_max: 0};
    unsafe { libc::getrlimit(libc::RLIMIT_FSIZE, &mut rlimit); }
    println!("{}, {}", rlimit.rlim_cur, rlimit.rlim_max);

    let mut rlimit = rlimit{rlim_cur: 0, rlim_max: 0};
    unsafe { libc::getrlimit(libc::RLIMIT_STACK, &mut rlimit); }
    println!("{}, {}", rlimit.rlim_cur, rlimit.rlim_max);
}
