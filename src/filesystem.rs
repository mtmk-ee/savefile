/// Functions for retrieving and creating directories used by this program.
///
/// Here is the structure of the directories used by this program:
///
/// ```
/// %LOCALAPPDATA%\savefile
/// ├── database.db
/// ├── profiles
/// │   ├── profile1.json
/// │   └── ...
/// └── saves
///    ├── profile1
///    │   ├── 1 (id)
///    │   │   └── <files>
///    │   ├── 2 (id)
///    │   │   └── <files>
///    │   └── ...
///    ├── profile2
///    └── ...
/// ```
use std::{
    io,
    path::{Path, PathBuf},
};

use crate::{error::Result, Id};

/// Creates the required directories for this program if they do not exist.
pub fn create_required_dirs() -> Result<()> {
    create_if_nonexistent(install_dir()?)?;
    create_if_nonexistent(profiles_dir()?)?;
    create_if_nonexistent(save_dir()?)?;
    Ok(())
}

/// Returns the directory where profiles are stored.
///
/// On Windows, this is `%LOCALAPPDATA%\savefile`.
///
/// This function will create the directory if it does not exist.
pub fn install_dir() -> Result<PathBuf> {
    let dir = dirs::data_local_dir()
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "could not find local data directory",
        ))?
        .join("savefile");
    create_if_nonexistent(&dir)?;
    Ok(dir)
}

/// Returns the path to the database.
pub fn database_path() -> Result<PathBuf> {
    Ok(install_dir()?.join("database.db"))
}

/// Returns the directory where profiles are stored.
pub fn profiles_dir() -> Result<PathBuf> {
    let dir = install_dir()?.join("profiles");
    create_if_nonexistent(&dir)?;
    Ok(dir)
}

/// Returns the path to a profile with the given name.
pub fn profile_path(name: impl AsRef<str>) -> Result<PathBuf> {
    Ok(profiles_dir()?.join(format!("{}.json", name.as_ref())))
}

/// Returns the directory where save files are stored.
pub fn save_dir() -> Result<PathBuf> {
    let dir = install_dir()?.join("saves");
    create_if_nonexistent(&dir)?;
    Ok(dir)
}

/// Returns the path to the backup directory for the given profile and ID.
pub fn backup_dir(profile: &str, id: Id) -> Result<PathBuf> {
    Ok(save_dir()?.join(profile).join(id.to_string()))
}

/// Expand the given glob pattern.
pub fn match_glob(pattern: &str) -> Result<Vec<PathBuf>> {
    let paths = glob::glob(pattern.as_ref()).expect("invalid glob pattern");
    let mut paths: Vec<PathBuf> = paths.filter_map(|p| p.ok()).collect();
    paths.sort();
    Ok(paths)
}

/// Create a directory if it does not exist.
fn create_if_nonexistent(dir: impl AsRef<Path>) -> Result<()> {
    match std::fs::create_dir(dir.as_ref()) {
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(()),
        other => other,
    }?;
    Ok(())
}
