use std::fs;
use std::process::Command;

static HEAD: &str = "./.git/refs/heads/master";

fn main() {
    let head_ref = match fs::read_to_string(HEAD) {
        Ok(sha) => sha,
        Err(_) => {
            eprintln!("Error: Could not read HEAD ref");
            return
        }
    };
    let head_sha = &head_ref[0.. &head_ref.len() - 1];
    let head_output = Command::new("git").args(["cat-file", "-p", head_sha])
        .output();
    println!("@ ----> {:?}", head_output);
}
