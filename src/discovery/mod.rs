use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const FOLDER_PATTERNS: &[&str] = &[
    "apache", "apache2", "httpd", "apache24",
    "xampp", "lampp", "laragon", "wamp", "mamp",
];

const LOG_FILENAMES: &[&str] = &[
    "access.log", "access_log", "apache-access.log",
    "apache_access.log", "access.log.1",
];

#[cfg(target_os = "windows")]
const SEARCH_ROOTS: &[&str] = &[
    "C:\\", "D:\\", "C:\\xampp", "C:\\laragon",
    "C:\\Apache24", "C:\\wamp", "C:\\wamp64",
];

#[cfg(not(target_os = "windows"))]
const SEARCH_ROOTS: &[&str] = &[
    "/var/log", "/opt", "/usr/local", "/etc",
    "/srv", "/home",
];

fn is_apache_folder(name: &str) -> bool {
    let lower = name.to_lowercase();
    FOLDER_PATTERNS.iter().any(|p| lower.contains(p))
}

fn is_log_file(name: &str) -> bool {
    let lower = name.to_lowercase();
    LOG_FILENAMES.iter().any(|f| lower == *f)
}

pub fn find_log_files() -> Vec<PathBuf> {
    let mut results = Vec::new();

    for root in SEARCH_ROOTS {
        let root_path = Path::new(root);
        if !root_path.exists() {
            continue;
        }

        for entry in WalkDir::new(root_path)
            .max_depth(6)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !is_apache_folder(name) {
                        continue;
                    }
                }
            }

            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if is_log_file(name) {
                        results.push(path.to_path_buf());
                    }
                }
            }
        }
    }

    results
}
