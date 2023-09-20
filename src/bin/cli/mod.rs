use std::path::{Path, PathBuf};

use savefile::{
    backup, delete_all_backups, delete_one_backup,
    error::{BackupError, ProfileError, Result},
    filesystem::{profile_path, profiles_dir, save_dir},
    list_profiles, Database, Id, Profile,
};

use crate::cli::{
    display::{BackupList, ProfileList},
    util::path_str,
};

use self::util::confirm;

pub mod args;
mod display;
mod util;

/// Print a list of installed profiles.
///
/// If `prefix` is given, only profiles with names starting with `prefix` will be listed.
pub fn print_profiles(prefix: Option<String>) -> Result<()> {
    let profiles = find_profile(prefix.as_deref())?;
    println!("{}", ProfileList(profiles).to_string());
    Ok(())
}

/// Open the directory where profiles are stored using the default program.
pub fn open_profiles_dir() -> Result<()> {
    let dir = profiles_dir()?;
    open::that(dir).expect("failed to open profiles directory");
    Ok(())
}

/// Open the profile with the given name using the default program.
pub fn edit_profile(name: &str) -> Result<()> {
    let path = profile_path(&name)?;
    if !path.exists() {
        Err(ProfileError::NoSuchProfile(path.clone()))?;
    }
    open::that(path).expect("failed to open profile");
    Ok(())
}

/// Create a new profile with the given name.
pub fn create_profile(name: &str, edit: bool) -> Result<()> {
    let path = profile_path(&name)?;
    match Profile::open(&path) {
        Ok(_) => Err(ProfileError::AlreadyExists)?,
        Err(_) => {
            Profile::new("INSERT").save(&path)?;
            println!("created profile {} at {:?}", name, path);
            if edit {
                open::that(path).expect("failed to open profile");
            }
            Ok(())
        }
    }
}

/// Delete the profile with the given name.
pub fn delete_profile(name: &str) -> Result<()> {
    let profile_path = profile_path(&name)?;
    if !profile_path.exists() {
        Err(ProfileError::NoSuchProfile(profile_path.clone()))?;
    }
    if confirm("Removing a profile will remove all its backups. Continue?") {
        let db = Database::open_default()?;
        delete_all_backups(&db, name)?;
        std::fs::remove_file(profile_path)?;
    }
    Ok(())
}

/// Find all profiles with names starting with `prefix`.
///
/// If `prefix` is `None`, all profiles will be returned.
pub fn find_profile(prefix: Option<&str>) -> Result<Vec<PathBuf>> {
    let mut profiles = list_profiles()?;
    if let Some(prefix) = prefix {
        let file_stem = |p: &Path| {
            p.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_owned())
                .unwrap()
        };
        profiles.retain(|(path, _)| file_stem(path).starts_with(prefix));
    }
    Ok(profiles.into_iter().map(|(path, _)| path).collect())
}

/// Immediately create a backup for the given profile.
pub fn create_backup(name: &str) -> Result<()> {
    let db = Database::open_default()?;
    let profile = Profile::open(&profile_path(&name)?)?;
    let id = backup(&db, &profile, &name)?;
    let save_dir = save_dir()?.join(id.to_string());
    println!("created backup {} for profile {}", id, name);
    println!("saved to {:?}", path_str(save_dir));
    Ok(())
}

/// Restore the given backup, or the latest backup if `id` is `None`.
pub fn restore_backup(name: &str, id: Option<Id>) -> Result<()> {
    if !confirm("This will overwrite your current files. Continue?")
        || !confirm("Is the watcher currently stopped?")
    // TODO: check lock file
    {
        return Ok(());
    }
    let db = Database::open_default()?;
    let id = match id {
        Some(id) => id,
        None => db
            .backup_table(name)?
            .latest()
            .ok_or(BackupError::BackupsEmpty)?
            .id(),
    };
    savefile::restore_backup(&db, name, id)
}

/// Delete one or all backups for the given profile.
///
/// First prompts the user for confirmation.
///
/// If `id` is given, only the backup with the given ID will be deleted.
/// Otherwise, all backups for the given profile will be deleted.
pub fn delete_backup(profile_name: &str, id: Option<Id>) -> Result<()> {
    if !confirm("This will delete the backup(s) permanently. Continue?") {
        return Ok(());
    }
    let db = Database::open_default()?;
    match id {
        Some(id) => delete_one_backup(&db, profile_name, id),
        None => delete_all_backups(&db, profile_name),
    }
}

/// Print a table of backups for the given profile.
pub fn print_backups(profile_name: &str, count: Option<usize>) -> Result<()> {
    let profile = Profile::open(&profile_path(profile_name)?)?;
    let db = Database::open_default()?;
    let backups = db.backup_table(profile_name)?.select_all();
    let count = count.unwrap_or(backups.len());
    let table = BackupList::new(profile, backups[..count].to_vec()).to_string();
    println!("{}", table);
    println!("Displayed {} of {} backups", count, backups.len());
    Ok(())
}

/// Delete all but the most recent `count` backups for the given profile.
pub fn retain_backups(profile_name: &str, count: usize) -> Result<()> {
    let msg = format!("Delete all but the {count} most recent backup(s)?");
    if !confirm(&msg) {
        return Ok(());
    }
    let db = Database::open_default()?;
    let backup_table = db.backup_table(profile_name)?;
    let mut backups = backup_table.select_all();
    backups.sort_by_key(|b| b.timestamp());
    backups.reverse();
    let to_delete = backups
        .iter()
        .skip(count)
        .map(|b| b.id())
        .collect::<Vec<_>>();
    if to_delete.is_empty() {
        println!("No backups to delete");
    } else {
        println!("Deleting {} backup(s)", to_delete.len());
        for id in to_delete {
            println!("Deleting backup {}", id);
            delete_one_backup(&db, profile_name, id)?;
        }
    }
    Ok(())
}

/// Run the filesystem watcher for the given profile.
///
/// This will watch the profile's base directory for changes and automatically
/// create a backup when a change to the requested files is detected.
pub fn run_watcher(profile_name: &str) -> Result<()> {
    let profile = Profile::open(profile_path(&profile_name)?)?;
    let db = Database::open_default()?;
    savefile::watch(&db, &profile, &profile_name)
}
