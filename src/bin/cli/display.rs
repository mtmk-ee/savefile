use std::path::PathBuf;

use savefile::{Backup, Profile};
use tabled::{builder::Builder, settings::Style};

/// A list of backups.
///
/// Primarily used for displaying backups in a table.
pub struct BackupList {
    profile: Profile,
    backups: Vec<Backup>,
}

impl BackupList {
    pub fn new(profile: Profile, backups: Vec<Backup>) -> Self {
        Self { profile, backups }
    }
}

impl ToString for BackupList {
    fn to_string(&self) -> String {
        let mut table = Builder::new();
        table.set_header(vec![
            "ID".to_owned(),
            "Timestamp".to_owned(),
            "Path".to_owned(),
        ]);
        self.backups.iter().for_each(|backup| {
            table.push_record(vec![
                backup.id().to_string(),
                backup.timestamp().to_string(),
                self.profile
                    .base()
                    .join(&backup.tag())
                    .display()
                    .to_string()
                    .replace("\\", "/"),
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
