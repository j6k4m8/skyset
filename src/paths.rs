use std::path::PathBuf;

pub fn normalize_path(input: Option<PathBuf>) -> PathBuf {
    let canonical = default_config_path();

    let expanded = input.map(expand_tilde).unwrap_or_else(|| canonical.clone());
    if expanded == home_dir() || expanded == home_dir().join(".config") {
        return canonical;
    }
    if expanded == home_dir().join(".config/skyset") {
        return canonical;
    }
    if expanded.is_dir() {
        return expanded.join("latest.yml");
    }

    expanded
}

pub fn default_config_path() -> PathBuf {
    home_dir().join(".config/skyset/latest.yml")
}

pub fn expand_tilde(path: PathBuf) -> PathBuf {
    let path_str = path.to_string_lossy();
    if path_str == "~" {
        return home_dir();
    }

    if let Some(stripped) = path_str.strip_prefix("~/") {
        return home_dir().join(stripped);
    }

    path
}

pub fn home_dir() -> PathBuf {
    directories::BaseDirs::new()
        .map(|dirs| dirs.home_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("/"))
}
