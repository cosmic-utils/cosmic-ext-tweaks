use std::{fs, io, path::Path};

use cosmic::cosmic_config::get_config_dir;

/// Remove local overrides while preserving the directory watched by COSMIC.
pub fn reset_cosmic_config(app_id: &str) {
    if let Some(root) = get_config_dir() {
        let dir = root.join("cosmic").join(app_id).join("v1");
        if let Err(error) = clear_config_dir(&dir) {
            log::error!("Failed to reset config for {app_id}: {error}");
        }
    }
}

fn clear_config_dir(dir: &Path) -> io::Result<()> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };

    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            fs::remove_dir_all(entry.path())?;
        } else {
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}
