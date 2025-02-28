use std::process::Command;
use clap::{Arg, Command as ClapCommand};

fn main() {
    let matches = ClapCommand::new("minigit")
        .version("0.1.0")
        .about("A simple CLI tool to combine git add, commit, and push")
        .arg(
            Arg::new("message")
                .help("Commit message")
                .required(true)
                .index(1),
        )
        .get_matches();

    let commit_message = matches.get_one::<String>("message").unwrap();
    
    println!("Running git add .");
    let add_status = Command::new("git")
        .args(["add", "."])
        .status()
        .expect("Failed to execute git add command");
    
    if !add_status.success() {
        println!("Error: git add command failed");
        return;
    }
    
    println!("Running git commit -m \"{}\"", commit_message);
    let commit_status = Command::new("git")
        .args(["commit", "-m", commit_message])
        .status()
        .expect("Failed to execute git commit command");
    
    if !commit_status.success() {
        println!("Error: git commit command failed");
        return;
    }
    
    println!("Running git push");
    let push_status = Command::new("git")
        .arg("push")
        .status()
        .expect("Failed to execute git push command");
    
    if !push_status.success() {
        println!("Error: git push command failed");
        return;
    }
    
    println!("Success! All git commands completed.");
}
