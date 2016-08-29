extern crate lispy;

fn main() {
    let status = lispy::run();
    std::process::exit(status);
}
