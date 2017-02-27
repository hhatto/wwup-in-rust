extern crate prctl;

fn main() {
    prctl::set_name("wwup-in-rust process");
}
