use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("OUT_DIR: {}", out_dir);

    let target_dir = PathBuf::from(out_dir).join("../../../../../target");
    println!("Target directory: {}", target_dir.display());

    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.display());

    let input_dir = current_dir.join("input");
    println!("Input directory: {}", input_dir.display());

    fs::create_dir_all(&target_dir).unwrap();

    copy_dir_recursively(&input_dir, &target_dir).unwrap();
}

fn copy_dir_recursively(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursively(&path, &dest_path)?;
        } else {
            println!("Copying file: {} to {}", path.display(), dest_path.display());
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}