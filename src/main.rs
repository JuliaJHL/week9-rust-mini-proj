use std::fs::{create_dir_all, read_dir, rename};
use std::path::{Path};

fn main() {
    // read directory path from command line
    let dir_path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: file-organizer <directory-path>");
        std::process::exit(1);
    });

    // read directory
    let dir_path = Path::new(&dir_path);
    let files = read_dir(dir_path).unwrap();

    // organize files
    for file in files {
        let file_path = file.unwrap().path();
        if file_path.is_file() {
            let extension = file_path.extension();
            if let Some(ext) = extension {
                let ext_str = ext.to_string_lossy().to_string();
                let dest_dir_path = dir_path.join(&ext_str);
                create_dir_all(&dest_dir_path).unwrap();

                let file_name = file_path.file_name().unwrap();
                let dest_file_path = dest_dir_path.join(file_name);
                rename(&file_path, &dest_file_path).unwrap();
            }
        }
    }
}
