use std::os::unix::process::parent_id;
use std::process;
fn main() {
    println!("Child current pid: {:?}", process::id());
    println!("Child PID: {}", parent_id());
    println!("Hello, world!");
}
