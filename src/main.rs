use std::fs;
use rayon::prelude::*;

fn process_directory(directory_path: &std::path::Path) -> std::io::Result<()> {
    let walker = fs::read_dir(directory_path)?;
    let paths: Vec<_> = walker.map(|entry| entry.unwrap().path()).collect();

    paths.par_iter().for_each(|entry_path| {
        println!("{}", entry_path.display());
        if entry_path.is_dir() {
            if let Err(err) = process_directory(entry_path) {
                eprintln!("Error processing directory entry {:?}: {}", entry_path, err);
            }
        }
    });

    Ok(())
}

fn main() -> std::io::Result<()> {
    let directory_path = std::path::PathBuf::from("C:\\");

    if !directory_path.exists() {
        eprintln!("Directory does not exist");
        return Ok(());
    }

    process_directory(&directory_path)?;

    Ok(())
}
