use std::{
    fs::File,
    process::{Command, Stdio},
};

fn main() {
    // Create watcher process

    let output_file = File::create("test.out").expect("Failed to create output file");
    let _ = Command::new("/usr/bin/nohup")
        .arg("/home/harristemuri/Projects/recap/target/debug/watcher")
        .stdout(Stdio::from(output_file))
        .stderr(Stdio::null())
        .spawn()
        .expect("couldn't spawn watcher");

    // // fork to fish shell

    // let mut sh = Command::new("fish").spawn().expect("cant open fish");
    // let output = sh.wait().unwrap();
    // println!("pid: {:?}", sh.id());
    // println!("{:?}", output)
}
