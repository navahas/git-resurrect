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
    println!("ref@ ----> {}", head_ref);

    let head_split: Vec<_> = head_ref.split("").collect();
    println!("head@ ----> {:?}", head_split);
    let folder_split: &String = &head_split[1..3].join("");
    let file_split: &String = &head_split[3 .. head_split.len() - 2].join("");
    println!("folder: {:?}, file: {:?}", folder_split, file_split);
}
