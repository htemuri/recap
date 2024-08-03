use core::str;
use nix::libc::{
    SYS_clone3, SYS_fork, SYS_read, SYS_write, PTRACE_EVENT_CLONE, PTRACE_EVENT_FORK,
    PTRACE_EVENT_VFORK, PTRACE_O_TRACEFORK, SIGTRAP,
};
use nix::sys::ptrace::Options;
use nix::sys::wait::WaitStatus;
use nix::sys::{ptrace, wait};
use nix::unistd::Pid;
use std::fmt::Debug;
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
    // let strpid = "736617";
    let pid = Pid::from_raw(strpid.parse().expect("failed to parse str to pid"));
    println!("Watcher received PID: {}", strpid);
    // let pid = Pid::from_raw("90684".parse().unwrap());
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
    let _ = ptrace::setoptions(
        pid,
        Options::PTRACE_O_TRACESYSGOOD
            | Options::PTRACE_O_TRACEEXIT
            | Options::PTRACE_O_TRACEEXEC
            | Options::PTRACE_O_TRACEFORK
            | Options::PTRACE_O_TRACEVFORK
            | Options::PTRACE_O_TRACECLONE,
    )
    .expect("Failed to set option");
    ptrace::syscall(pid, None).expect("Error waiting for syscall");

    let mut index = 0;

    loop {
        // Wait for the process to stop
        let res = wait::waitpid(pid, None).expect("Failed to wait for the process");
        println!("Wait result: {:?}", res);
        let reg: nix::libc::user_regs_struct = ptrace::getregs(pid).expect("failed to get regs");

        match res {
            WaitStatus::PtraceEvent(_, _, event) => {
                match event {
                    PTRACE_EVENT_CLONE | PTRACE_EVENT_FORK | PTRACE_EVENT_VFORK => {
                        let new_pid = ptrace::getevent(pid).unwrap();
                        println!("Attaching to forked process with id: {}", new_pid);
                        // ptrace::cont(Pid::from_raw(new_pid as i32), None)
                        //     .expect("failed to continue parent process after being forked");
                        attach_and_read_output(Pid::from_raw(new_pid as i32));
                    }
                    _ => continue,
                }
            }
            _ => {
                if reg.orig_rax == SYS_read.try_into().unwrap()
                    || reg.orig_rax == SYS_write.try_into().unwrap()
                    || reg.orig_rax == reg.rax
                {
                    let mut string = String::new();
                    for i in 0..reg.rdx {
                        let data = ptrace::read(pid, (reg.rsi + i) as *mut c_void)
                            .expect("Failed to read from memory");
                        string.push((data & 0xff) as u8 as char);
                    }
                    // every syscall is supposed to run twice apparently
                    // so removing the duplicate when printing
                    if index % 2 == 0 {
                        print!("{}", string);
                    }
                    index += 1;
                }
            }
        }
        // if res
        //     == WaitStatus::PtraceEvent(pid, nix::sys::signal::Signal::SIGTRAP, PTRACE_EVENT_VFORK)
        // {
        //     let new_pid = ptrace::getevent(pid).unwrap();
        //     println!("Attaching to forked process with id: {}", new_pid);
        //     // ptrace::cont(Pid::from_raw(new_pid as i32), None)
        //     //     .expect("failed to continue parent process after being forked");
        //     attach_and_read_output(Pid::from_raw(new_pid as i32));
        // }

        // println!(
        //     "registers: \nRDI: {:?}\nRSI: {:?}\nRDX: {:?}",
        //     reg.rdi, reg.rsi, reg.rdx
        // );
        ptrace::syscall(pid, None).expect("Error waiting for syscall");
    }

    // Read data from the process memory (example: read the first few bytes of the stack)
    // let address: *mut c_void = 0x7fffffffe000 as *mut c_void; // Adjust this address as needed

    // Continue the target process
    // ptrace::cont(pid, None).expect("Failed to continue the target process");
    // println!("Continued process with PID {}", pid);

    // Detach from the target process
    ptrace::detach(pid, None).expect("Failed to detach from the target process");
    println!("Detached from process with PID {}", pid);
}
