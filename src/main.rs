use std::fs;
use std::process::{Command, Output};

static HEAD: &str = "./.git/refs/heads/master";

#[derive(Debug)]
enum GtoType {
    Blob,
    Tree
}

impl GtoType {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "blob" => Some(GtoType::Blob),
            "tree" => Some(GtoType::Tree),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct GitTreeObject {
    mode: u32,
    gto_type: GtoType,
    sha: String,
    file_name: String
}

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
    let mut git_tree_objects: Vec<GitTreeObject> = Vec::new();
    for line in tree_sha_content.lines() {
        // let git_tree_object: Vec<GitTreeObject> = vec![];
        let gto_line: Vec<&str> = line.split_whitespace().collect();

        let mode = gto_line[0].parse::<u32>().unwrap();
        let gto_type_str = gto_line[1];
        let sha = gto_line[2].parse::<String>().unwrap();
        let file_name = gto_line[3].parse::<String>().unwrap();

        let gto_struct = GitTreeObject {
            mode,
            gto_type: GtoType::from_str(gto_type_str).expect(""),
            sha,
            file_name
        };

        git_tree_objects.push(gto_struct);
    }
    println!("{:?}", git_tree_objects);
}
