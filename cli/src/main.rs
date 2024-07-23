use std::{
    fs::File,
    process::{self, Command, Stdio},
};

fn main() {
    println!("current pid: {:?}", process::id());
    let output_file = File::create("test.out").expect("Failed to create output file");
    let _ = Command::new("/usr/bin/nohup")
        .arg("/home/harristemuri/Projects/recap/target/debug/watcher")
        .stdout(Stdio::from(output_file))
        .stderr(Stdio::null())
        .spawn()
        .expect("couldn't spawn watcher");
}
