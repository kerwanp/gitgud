use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn is_git_repository(root: &str) -> bool {
    fs::exists(Path::new(&format!("{}/{}", root, ".git"))).unwrap_or(false)
}

pub fn is_initialized(root: &str) -> bool {
    fs::exists(Path::new(&format!("{}/{}", root, ".gitgud"))).unwrap_or(false)
}

pub fn find_git_root(cwd: &str) -> Option<PathBuf> {
    let mut current_path = Path::new(cwd).to_path_buf();

    loop {
        if current_path.join(".git").exists() {
            return Some(current_path);
        }

        if !current_path.pop() {
            break;
        }
    }

    None
}
