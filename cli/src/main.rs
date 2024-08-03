use std::{
    fs::File,
    io::{self, Error, Write},
    os::unix::process::{self, CommandExt},
    process::{id, Command, Stdio},
    thread::sleep,
    time::Duration,
};

use nix::unistd::mkfifo;
use nix::{
    sys::{ptrace, stat},
    unistd::Pid,
};
use spawn_ptrace::CommandPtraceSpawn;

fn main() {
    // create FIFO for communication
    let fifopath = "/home/harristemuri/Projects/recap/comms";
    match mkfifo(fifopath, stat::Mode::S_IRWXU) {
        Err(e) => {
            println!("Error creating FIFO: {}", e)
        }
        Ok(_) => println!("Created FIFO"),
    }

    // Create watcher process
    let output_file = File::create("test.out").expect("Failed to create output file");
    let _ = Command::new("/usr/bin/nohup")
        .arg("/home/harristemuri/Projects/recap/target/debug/watcher")
        .stdout(Stdio::from(output_file.try_clone().unwrap()))
        .stderr(Stdio::from(output_file))
        .spawn()
        .expect("couldn't spawn watcher");

    {
        let mut buff = File::options().write(true).open(fifopath).unwrap();
        println!("CLI Opened FIFO");
        let res: usize = buff.write(&id().to_string().as_bytes()).unwrap();
        println!("Wrote {} bytes to FIFO", res);
    }
    println!("CLI PID: {}", id());
    let mut x = 1;
    loop {
        println!("{}", x);
        sleep(Duration::from_secs(2));
        x += 1;
        if x == 3 {
            break;
        }
    }
    // // fork to fish shell
    let mut sh = unsafe {
        Command::new("ls")
            .pre_exec(|| {
                // Opt-in to ptrace.
                ptrace::traceme().map_err(|e| match e {
                    _ => Error::new(io::ErrorKind::Other, "unknown PTRACE_TRACEME error"),
                })
            })
            .spawn()
            .expect("cant open fish")
    };
    let output = sh.wait().unwrap();
    println!("pid: {:?}", sh.id());
    println!("{:?}", output)
}
