#![allow(dead_code)]

use std::{env, fs};
use std::path::{Path, PathBuf};

pub fn is_windows() -> bool {
    env::consts::OS == "windows"
}

pub fn fix_path(path: String) -> String {
    // 只有 Windows 使用反斜杠，单独判断这种情况即可！
    if is_windows() {
        path.replace("/", "\\")
    } else {
        path.replace("\\", "/")
    }
}

#[allow(dead_code)]
pub fn join_path(path: String, name: String) -> String {
    fix_path(
        format!("{}/{}", path.trim(), name.trim())
    )
}

pub fn remove_if_exist(path: String) {
    let path = path.as_str();
    if Path::new(path).exists() {
        std::fs::remove_file(path).expect("could not remove file");
    }
}

#[allow(dead_code)]
pub fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap()
}

// 其实就是简单的记录一下怎么用
pub fn create_if_missing(path: &str) -> std::io::Result<()> {
    let path = PathBuf::from(path);

    return fs::create_dir_all(path);
}

pub fn path_buf2str(path: PathBuf) -> String {
    String::from(path.to_str().unwrap())
}

pub fn get_file_name_suffix(full_file_name: String) -> (String, String) {
    match full_file_name.rfind(".") {
        Some(i) => (
            String::from(&full_file_name[0..i]),
            String::from(&full_file_name[i + 1..full_file_name.len()]),
        ),
        None => (full_file_name, String::new()),
    }
}