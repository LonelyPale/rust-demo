use std::os::unix::fs::MetadataExt;
use std::{env, fs};
use walkdir::WalkDir;

#[test]
fn dir1() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    println!("current_dir: {:?}", current_dir);

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let metadata = entry.metadata()?;
        let size = metadata.size();
        let mut tag = "";

        if metadata.is_file() {
            tag = "file";
        } else if metadata.is_dir() {
            tag = "dir";
        } else {
            tag = "unknown";
        }
        println!("{}: {:?} {:?} {}", tag, name, path, size);
    }

    Ok(())
}

#[test]
fn dir2() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    println!("current_dir: {:?}", current_dir);

    for entry in WalkDir::new(current_dir) {
        let entry = entry?;
        println!("{}", entry.path().display());
    }

    Ok(())
}
