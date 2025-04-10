use std::fs;
static HEAD: &str = "./.git/refs/heads/master";

fn main() {
    let head_ref = match fs::read_to_string(HEAD) {
        Ok(sha) => sha,
        Err(_) => {
            eprintln!("Error: Could not read HEAD ref");
            return
        }
    };

    println!("@ ----> {}", head_ref);
}
