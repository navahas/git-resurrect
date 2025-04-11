use std::fs;
use std::process::{Command, Output};

static HEAD: &str = "./.git/refs/heads/master";

fn git_cat_file(sha: &str) -> Output {
    Command::new("git")
        .args(["cat-file", "-p", &sha])
        .output()
        .expect("Error: Unable to read HEAD content")
}

// fn git_resurrect(sha: &str) -> Output {
// }

fn main() {
    let head_ref = match fs::read_to_string(HEAD) {
        Ok(sha) => sha.trim().to_string(),
        Err(_) => {
            eprintln!("Error: Could not read HEAD ref");
            return
        }
    };

    let head_output = git_cat_file(&head_ref);

    let head_content = String::from_utf8_lossy(&head_output.stdout);
    let head_tree_content = &head_content
        .lines()
        .next()
        .unwrap_or("");

    let head_tree_sha = head_tree_content
        .split(" ")
        .nth(1)
        .unwrap_or("");

    let head_tree_output = git_cat_file(&head_tree_sha);

    let tree_sha_content = String::from_utf8_lossy(&head_tree_output.stdout);
    for line in tree_sha_content.lines() {
        println!("@ ----> {}", line);
    }
}
