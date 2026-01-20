use std::{fs, io, path::Path};

use crate::model::Skyset;

pub struct ReadOutcome {
    pub skyset: Skyset,
    pub raw: String,
}

pub fn load_initial_state(path: &Path) -> ReadOutcome {
    match read_skyset(path) {
        Ok(outcome) => outcome,
        Err(_err) => ReadOutcome {
            skyset: Skyset::default(),
            raw: String::new(),
        },
    }
}

pub fn read_skyset(path: &Path) -> io::Result<ReadOutcome> {
    let content = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Err(err),
        Err(err) => return Err(err),
    };

    if content.trim().is_empty() {
        return Ok(ReadOutcome {
            skyset: Skyset::default(),
            raw: String::new(),
        });
    }

    match serde_yaml::from_str::<Skyset>(&content) {
        Ok(skyset) => Ok(ReadOutcome {
            skyset,
            raw: content,
        }),
        Err(_err) => Ok(ReadOutcome {
            skyset: Skyset::default(),
            raw: content,
        }),
    }
}

pub fn write_skyset(path: &Path, contents: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let tmp_path = path.with_extension("yml.tmp");
    fs::write(&tmp_path, contents)?;
    fs::rename(&tmp_path, path)?;
    Ok(())
}
