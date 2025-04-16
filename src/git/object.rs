use std::process::{Command, Output};

#[derive(Debug, PartialEq)]
pub enum GtoType {
    Blob,
    Tree
}

impl GtoType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "blob" => Some(GtoType::Blob),
            "tree" => Some(GtoType::Tree),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct GitTreeObject {
    pub mode: u32,
    pub gto_type: GtoType,
    pub sha: String,
    pub file_name: String,
    pub parent: Option<String>
}

pub fn git_cat_file(sha: &str) -> Output {
    Command::new("git")
        .args(["cat-file", "-p", &sha])
        .output()
        .expect("Error: Unable to read HEAD content")
}
