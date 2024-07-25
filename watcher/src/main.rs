use core::str;
use nix::sys::{ptrace, wait};
use nix::unistd::Pid;
use std::fs::File;
use std::io::Read;
use std::os::raw::c_void;
// use std::os::unix::process::parent_id;
// use std::process;
fn main() {
    // println!("Child current pid: {:?}", process::id());
    // println!("Child PID: {}", parent_id());
    // println!("Hello, world!");

    let fifopath = "/home/harristemuri/Projects/recap/comms";

    let mut fifo = File::open(fifopath).unwrap();
    println!("Opened FIFO");
    let mut pid = Vec::new();
    fifo.read_to_end(&mut pid).expect("failed to read fifo");
    let strpid = str::from_utf8(&pid).expect("failed to parse vec to string");
    let pid = Pid::from_raw(strpid.parse().expect("failed to parse str to pid"));
    println!("Watcher received PID: {}", strpid);

    attach_and_read_output(pid);

    // match ptrace::attach(pid) {
    //     Err(e) => println!("Error attaching to PID: {}", e),
    //     Ok(_) => println!("Attached to {}", strpid),
    // }
    // println!("Attached to pid");
    // let res = ptrace::getevent(pid).unwrap();
    // println!("res: {}", res)
    // // ptrace to attach to the process to redirect the fd output
}

fn attach_and_read_output(pid: Pid) {
    // Attach to the target process
    match ptrace::attach(pid) {
        Err(e) => println!("Error attaching to PID: {}", e),
        Ok(_) => println!("Attached to {}", pid),
    }

    // Wait for the process to stop
    wait::waitpid(pid, None).expect("Failed to wait for the process");
    // Read data from the process memory (example: read the first few bytes of the stack)
    let address: *mut c_void = 0x7fffffffe000 as *mut c_void; // Adjust this address as needed
    let data = ptrace::read(pid, address).expect("Failed to read from memory");
    println!("Data from process: {}", data);

    // Continue the target process
    ptrace::cont(pid, None).expect("Failed to continue the target process");
    println!("Continued process with PID {}", pid);

    // Detach from the target process
    ptrace::detach(pid, None).expect("Failed to detach from the target process");
    println!("Detached from process with PID {}", pid);
}
