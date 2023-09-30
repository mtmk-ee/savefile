use std::path::PathBuf;

use savefile::{filesystem::backup_dir, Backup};
use tabled::{builder::Builder, settings::Style};

use super::util::path_str;

/// A list of backups.
///
/// Primarily used for displaying backups in a table.
pub struct BackupList<'a> {
    profile_name: &'a str,
    backups: Vec<Backup>,
}

impl<'a> BackupList<'a> {
    pub fn new(profile_name: &'a str, backups: Vec<Backup>) -> Self {
        Self {
            profile_name,
            backups,
        }
    }
}

impl ToString for BackupList<'_> {
    fn to_string(&self) -> String {
        let mut table = Builder::new();
        table.set_header(vec![
            "ID".to_owned(),
            "Timestamp".to_owned(),
            "Path".to_owned(),
        ]);
        self.backups.iter().for_each(|backup| {
            let path = match backup_dir(&self.profile_name, backup.id()) {
                Ok(path) => path_str(&path),
                Err(_) => "(invalid)".to_owned(),
            };
            table.push_record(vec![
                backup.id().to_string(),
                backup.timestamp().to_string(),
                path,
            ]);
        });
        table.build().with(Style::ascii_rounded()).to_string()
    }
}

/// A list of profiles.
///
/// Primarily used for displaying profiles in a table.
pub struct ProfileList(pub Vec<PathBuf>);

impl ToString for ProfileList {
    fn to_string(&self) -> String {
        let mut table = Builder::new();
        table.set_header(vec!["Name".to_owned(), "Path".to_owned()]);
        self.0.iter().for_each(|path| {
            table.push_record(vec![
                path.file_stem().unwrap().to_str().unwrap().to_owned(),
                path.display().to_string().replace("\\", "/"),
            ]);
        });
        table.build().with(Style::ascii_rounded()).to_string()
    }
}
