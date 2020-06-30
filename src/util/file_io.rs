use std::fs;
use std::path::Path;

pub fn get_directory_list(directory_path:&str) -> Result<Vec<String>, std::io::Error> {
    let mut found_paths: Vec<String> = Vec::new();
    let path = Path::new(&directory_path);

    for entry in fs::read_dir(path)? {
        let found_path = entry?.path();
        if !found_path.is_dir() {
            if let Some(path) = found_path.to_str() {
                found_paths.push(String::from(path));
            }
        }
    }

    Ok(found_paths)
}
