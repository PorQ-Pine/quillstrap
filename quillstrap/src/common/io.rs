use std::{env::{self, current_dir}, fs::remove_dir_all, path::Path};

use crate::prelude::*;

pub fn read_file_str(path: String) -> Result<String, std::io::Error> {
    match std::fs::read_to_string(path.clone()) {
        Ok(x) => Ok(x),
        Err(x) => {
            error!("Failed to read file at path: \"{}\", error: {:?}", &path, x);
            Err(x)
        }
    }
}

pub fn copy_file(src: &str, dest: &str) -> Result<(), std::io::Error> {
    match std::fs::copy(&src, &dest) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!(
                "Failed to copy file from \"{}\" to \"{}\", error: {:?}",
                &src, &dest, e
            );
            Err(e)
        }
    }
}

pub fn remove_file(path: &str, error_out: bool) -> Result<(), std::io::Error> {
    match std::fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            if error_out {
                error!(
                    "Failed to remove file at path: \"{}\", error: {:?}",
                    &path, e
                );
            }
            Err(e)
        }
    }
}

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn dir_change(path: &str) {
    env::set_current_dir(path).expect(&format!("Failed to change directory to: {}, current path is: {}", path, current_dir().unwrap().to_string_lossy()))
}

pub fn mkdir_p(path: &str) {
    let dir = Path::new(path);
    std::fs::create_dir_all(dir).expect(&format!("Failed to create dir at: {}", path));
}

pub fn clean_dir(path: &str) {
    remove_dir_all(path).unwrap();
    mkdir_p(path);
}

pub fn dir_current() -> String {
    env::current_dir()
        .expect("Failed to get the current dir")
        .to_str()
        .unwrap()
        .to_string()
}

pub fn set_var(key: &str, val: &str) {
    unsafe {
        env::set_var(key, val);
    }
}

pub fn replace_string_file(file: &str, str_to_replace: &str, replace_with: &str) {
    let content = std::fs::read_to_string(file).unwrap();
    let new_content = content.replace(str_to_replace, replace_with);
    std::fs::write(file, new_content).unwrap();
}

pub fn copy_dir_content(src: &str, dst: &str) {
    if !path_exists(src) {
        panic!("There is no {} dir", src);
    }
    if !path_exists(dst) {
        panic!("There is no {} dir", dst);
    }
    fn copy_recursively(src: &Path, dst: &Path) {
        for entry in
            std::fs::read_dir(src).expect(&format!("Failed to read directory {}", src.display()))
        {
            let entry = entry.expect(&format!("Failed to get entry in {}", src.display()));
            let path = entry.path();
            let dest_path = dst.join(entry.file_name());

            if path.is_dir() {
                std::fs::create_dir_all(&dest_path).expect(&format!(
                    "Failed to create directory {}",
                    dest_path.display()
                ));
                copy_recursively(&path, &dest_path);
            } else {
                if let Some(parent) = dest_path.parent() {
                    std::fs::create_dir_all(parent).expect(&format!(
                        "Failed to create parent directory {}",
                        parent.display()
                    ));
                }
                std::fs::copy(&path, &dest_path).expect(&format!(
                    "Failed to copy file {} to {}",
                    path.display(),
                    dest_path.display()
                ));
            }
        }
    }

    copy_recursively(Path::new(src), Path::new(dst));
}

pub fn remove_files_recursive(dir: &str, target: &str) {
    let entries = std::fs::read_dir(dir).expect(&format!("Failed to read directory {}", dir));
    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_dir() {
            remove_files_recursive(path.to_str().unwrap(), target);
        } else if path.file_name().unwrap() == target {
            std::fs::remove_file(&path).expect(&format!("Deleted file {:?}", path));
        }
    }
}

pub fn append_to_file(path: &str, string: &str) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect(&format!(
            "Failed to open or create the file at path: {}",
            path
        ));
    file.write_all(string.as_bytes()).expect(&format!(
        "Failed to write the string to the file at path: {}",
        path
    ));
}

pub fn create_file_symlink(symlink_to: &str, path_of_symlink: &str) {
    run_command(&format!("ln -s {} {}", symlink_to, path_of_symlink), true).unwrap();
}
