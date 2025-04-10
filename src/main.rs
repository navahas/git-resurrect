use std::{fs, io};

fn main() -> io::Result<()> {
    let git_dir = fs::read_dir("./.git")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    println!("@ ----> {:?}", git_dir);
    Ok(())
}

