use std::process::Command;
use std::ffi::CString;

fn process_spawn() {
    let mut sleep_command = Command::new("sleep").arg("3").spawn().expect("sleep command error");
    let mut ls_command = Command::new("ls").arg("-l").spawn().expect("ls command error");

    let ret = sleep_command.wait().expect("sleep wait error");
    println!("sleep result={}", ret);
    let ret = ls_command.wait().expect("ls wait error");
    println!("ls result={}", ret);
}

fn libc_popen() {
    let msg = CString::new("some\ndata").unwrap();
    let command = CString::new("less").unwrap();
    let args = CString::new("w").unwrap();
    unsafe {
        let file = libc::popen(command.as_ptr(), args.as_ptr());
        libc::fwrite(msg.as_ptr() as *const libc::c_void, 1, msg.to_bytes().len(), file);
        libc::pclose(file);     // exec less -> input q -> process end
    };
}

fn main() {
    process_spawn();
    libc_popen();
}
