use std::io;
use std::process::{Command, Stdio};

pub fn get_input(prompt: &str) -> String {
    eprint!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

pub fn run_command(command: String) {
    println!("Running command: {}", &command);

    #[cfg(target_os = "windows")]
    let shell = ("powershell", "-Command"); // powershell
    //let shell = ("cmd", "/C"); // cmd

    #[cfg(not(target_os = "windows"))]
    let shell = ("sh", "-c");

    let mut child = Command::new(shell.0)
        .args(&[shell.1, &command])
        .stdin(Stdio::inherit()) // attach stdin for interactive input
        .stdout(Stdio::inherit()) // attach stdout for live output
        .stderr(Stdio::inherit()) // attach stderr for live errors
        .spawn()
        .expect("Failed to spawn process");

    child.wait().expect("Failed to wait on child");

    //let status = child.wait().expect("Failed to wait on child");
    // if status.success() {
    //     println!("✅ Command completed successfully!");
    // } else {
    //     eprintln!("❌ Command failed!");
    // }
}
