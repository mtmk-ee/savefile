use std::path::Path;

use chrono::Utc;

use crate::{
    database::Database,
    error::Result,
    filesystem::{backup_dir, profile_path, save_dir},
    profile::Profile,
};

pub type Timestamp = chrono::NaiveDateTime;
pub type Id = u32;

/// Lightweight representation of a single backup.
///
/// Note: The ID of each backup is unique to the profile,
/// meaning that two different profiles can have backups
/// with the same ID.
#[derive(Clone, Debug)]
pub struct Backup {
    /// The backup's ID.
    id: u32,
    /// The backup's tag. (unused)
    tag: String,
    /// The backup's time of creation.
    timestamp: Timestamp,
}

impl Backup {
    /// Create a new backup representation.
    ///
    /// This function is for internal use only.
    pub(crate) fn new(id: u32, tag: String, timestamp: Timestamp) -> Self {
        Self { id, tag, timestamp }
    }

    /// Returns the backup's ID.
    ///
    /// This ID is unique to the profile, not the entire database.
    /// The ID also corresponds to the backup's directory name.
    pub fn id(&self) -> Id {
        self.id
    }

    /// Returns the backup's tag.
    ///
    /// The tag is WIP, but is intended to be used as a human-readable
    /// description of the backup for easy restoration.
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// Returns the backup's timestamp.
    ///
    /// This is the time at which the backup was created.
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}

/// Create a backup of the given profile.
///
/// This function will create a new backup entry in the database and copy all
/// files specified by the profile into the backup directory.
pub fn backup(db: &Database, profile: &Profile, name: &str) -> Result<Id> {
    let id = db
        .backup_table(&name)?
        .insert("unused", &Utc::now().naive_utc())?
        .id();
    let backup_dir = backup_dir(name, id)?;
    std::fs::create_dir_all(&backup_dir)?;
    profile
        .expand_includes(true)?
        .into_iter()
        .try_for_each(|rel_src| {
            let dest = backup_dir.join(&rel_src);
            let abs_src = profile.base().join(&rel_src);
            copy(&abs_src, &dest)
        })?;
    Ok(id)
}

/// Delete the backup with the given ID.
///
/// This removes the backup from the database and deletes the backup's directory.
pub fn delete_one_backup(db: &Database, profile: &str, id: Id) -> Result<()> {
    let backup_table = db.backup_table(profile)?;
    let backup_dir = backup_dir(profile, id)?;
    backup_table.remove(id)?;
    std::fs::remove_dir_all(backup_dir)?;
    Ok(())
}

/// Delete all backups with the given ID.
///
/// This removes all backups from the database and deletes all backup directories.
pub fn delete_all_backups(db: &Database, profile: &str) -> Result<()> {
    let backup_table = db.backup_table(profile)?;
    let backup_dir = save_dir()?.join(profile);
    backup_table.drop()?;
    std::fs::remove_dir_all(backup_dir)?;
    Ok(())
}

/// Restore the backup with the given ID.
///
/// This function will copy all files from the backup directory into the profile's
/// base directory.
pub fn restore_backup(db: &Database, profile: &str, id: Id) -> Result<()> {
    // check that the backup exists
    let _ = db.backup_table(profile)?.select_id(id).expect("bad ID");
    let dest_dir = Profile::open(&profile_path(profile)?)?.base().to_owned();
    let src_dir = backup_dir(profile, id)?;
    copy_dir_contents(&src_dir, &dest_dir)?;
    Ok(())
}

/// Copy a file or directory from `src` to `dest`.
///
/// This function is non-recursive for directories.
fn copy(src: &Path, dest: &Path) -> Result<()> {
    if src.is_dir() {
        create_dirs(&dest)?;
    } else if !dest.exists() {
        create_dirs(dest.parent().expect("what??"))?;
        std::fs::copy(src, dest)?;
    }
    Ok(())
}

/// Copy the contents of a directory recursively from `src` to `dest`.
fn copy_dir_contents(src: &Path, dest: &Path) -> Result<()> {
    create_dirs(dest)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src = entry.path();
        let dest = dest.join(entry.file_name());
        if src.is_dir() {
            create_dirs(&dest)?;
            copy_dir_contents(&src, &dest)?;
        } else {
            copy(&src, &dest)?;
        }
    }
    Ok(())
}

/// Create all missing directories (if any) in the given path.
fn create_dirs(path: &Path) -> Result<()> {
    match std::fs::create_dir_all(path) {
        Err(e) if e.kind() != std::io::ErrorKind::AlreadyExists => Err(e)?,
        _ => {}
    }
    Ok(())
}
