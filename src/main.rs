use std::fs;
use std::process::{Command, Output};

static HEAD: &str = "./.git/HEAD";

#[derive(Debug, PartialEq)]
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
    file_name: String,
    parent: Option<String>
}

fn git_cat_file(sha: &str) -> Output {
    Command::new("git")
        .args(["cat-file", "-p", &sha])
        .output()
        .expect("Error: Unable to read HEAD content")
}


fn main() {
    let head_ref = match fs::read_to_string(HEAD) {
        Ok(sha) => sha.split_whitespace().nth(1).expect("").to_string(),
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

    let mut git_tree_objects: Vec<GitTreeObject> = Vec::new();

    let head_tree_output = git_cat_file(&head_tree_sha);
    let tree_sha_content = String::from_utf8_lossy(&head_tree_output.stdout);
    for line in tree_sha_content.lines() {
        let gto_line: Vec<&str> = line.split_whitespace().collect();

        let mode = gto_line[0].parse::<u32>().unwrap();
        let gto_type_str = gto_line[1];
        let sha = gto_line[2].parse::<String>().unwrap();
        let file_name = gto_line[3].parse::<String>().unwrap();

        let gto_type = match GtoType::from_str(gto_type_str) {
            Some(t) => t,
            None => continue
        };

        let gto_struct = GitTreeObject {
            mode,
            gto_type,
            sha,
            file_name,
            parent: None
        };

        git_tree_objects.push(gto_struct);
    }
    
    let pwd = std::env::current_dir().unwrap();

    let mut gto_leaves: Vec<GitTreeObject> = vec![];
    for gto_object in &git_tree_objects {
        if gto_object.gto_type == GtoType::Tree {
            let tree_dir_name = &gto_object.file_name;
            let tree_dir_path = format!("{}/{}", pwd.display(), tree_dir_name);
            // println!("Created _Dir: ----> {}", tree_dir_path);
            fs::create_dir(tree_dir_path).unwrap();

            let tree_dir_sha = &gto_object.sha;
            let leave_tree_output = git_cat_file(&tree_dir_sha);
            let leave_sha_content = String::from_utf8_lossy(&leave_tree_output.stdout);
            for line in leave_sha_content.lines() {
                let gto_line: Vec<&str> = line.split_whitespace().collect();

                let mode = gto_line[0].parse::<u32>().unwrap();
                let gto_type_str = gto_line[1];
                let sha = gto_line[2].parse::<String>().unwrap();
                let file_name = gto_line[3].parse::<String>().unwrap();

                let gto_struct = GitTreeObject {
                    mode,
                    gto_type: GtoType::from_str(gto_type_str).expect(""),
                    sha,
                    file_name,
                    parent: Some(tree_dir_name.clone())
                };

                gto_leaves.push(gto_struct);
            }
        } else {
            let blob_file_name = &gto_object.file_name;
            let blob_dir_path = format!("{}/{}", pwd.display(), blob_file_name);
            // println!("Created File: ----> {}", blob_dir_path);
            let blob_sha = &gto_object.sha;
            let blob_content = git_cat_file(blob_sha);
            fs::write(&blob_dir_path, &blob_content.stdout).unwrap_or_else(|e| {
                eprintln!("Failed to write file content {} :\n{}", &blob_dir_path, e);
            });
        } 
    }

    while let Some(leave) = gto_leaves.pop() {
        if leave.gto_type == GtoType::Tree {
            let tree_dir_name = &leave.parent.expect("");
            let tree_file_name = &leave.file_name;
            let tree_dir_path = format!("{}/{}", tree_dir_name, tree_file_name);
            // println!("Created _Dir: ----> {}", tree_dir_path);
            fs::create_dir(&tree_dir_path).unwrap_or_else(|e| {
                eprintln!("Failed to create directory {}: {}", tree_dir_path, e);
            });

            let tree_dir_sha = &leave.sha;
            let leave_tree_output = git_cat_file(&tree_dir_sha);
            let leave_sha_content = String::from_utf8_lossy(&leave_tree_output.stdout);
            for line in leave_sha_content.lines() {
                let gto_line: Vec<&str> = line.split_whitespace().collect();

                let mode = gto_line[0].parse::<u32>().unwrap();
                let gto_type_str = gto_line[1];
                let sha = gto_line[2].parse::<String>().unwrap();
                let file_name = gto_line[3].parse::<String>().unwrap();

                let gto_type = match GtoType::from_str(gto_type_str) {
                    Some(t) => t,
                    None => {
                        println!("{}", line);
                        continue
                    }
                };

                let gto_leave_struct = GitTreeObject {
                    mode,
                    gto_type,
                    sha,
                    file_name,
                    parent: Some(tree_dir_path.clone())
                };

                gto_leaves.push(gto_leave_struct);
            }
        } else if leave.gto_type == GtoType::Blob {
            let blob_dir_name = &leave.parent.expect("");
            let blob_file_name = &leave.file_name;
            let blob_dir_path = format!("{}/{}", blob_dir_name, blob_file_name);
            // println!("Created File: ----> {}", blob_dir_path);
            let blob_sha = &leave.sha;
            let blob_content = git_cat_file(blob_sha);
            fs::write(&blob_dir_path, &blob_content.stdout).unwrap_or_else(|e| {
                eprintln!("Failed to write file content {} :\n{}", &blob_dir_path, e);
            });
        } else {
            println!("{:?}", leave);
        }
    }

}
