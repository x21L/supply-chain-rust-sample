use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    let output = Command::new("ip")
        .arg("a")
        .output()
        .expect("Failed to execute command");

    let mut file = File::create("output.txt")
        .expect("Could not create file");

    file.write_all(output.stdout.as_slice())
        .expect("Could not write to file");
    file.write_all(b"\nTrust me, this is very secure.")
        .expect("Could not write to file");
}