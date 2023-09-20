use savefile::Id;

/// Top-level CLI argument parser
#[derive(clap::Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: SubCmd,
}

/// Top-level CLI subcommands
#[derive(clap::Subcommand)]
pub enum SubCmd {
    /// Manage profiles
    #[clap(subcommand)]
    Profile(ProfileCmd),
    /// Manage backups
    #[clap(subcommand)]
    Backup(BackupCmd),
    /// Automatically back up files
    Watch {
        /// Name of the profile to watch
        #[clap(short, long)]
        name: String,
    },
}

/// "profile" subcommand
#[derive(clap::Subcommand)]
pub enum ProfileCmd {
    /// List all profiles
    List {
        /// Optional prefix to filter profiles by
        #[clap(short, long)]
        prefix: Option<String>,
    },
    /// Add a new profile
    Create {
        /// Name of the profile to add
        #[clap(short, long)]
        name: String,
        /// Open the profile in an editor after creating it
        #[clap(short, long, default_value_t = false)]
        edit: bool,
    },
    /// Remove a profile
    Delete {
        /// Name of the profile to remove
        #[clap(short, long)]
        name: String,
    },
    /// Browse profiles in a file manager
    Browse,
    /// Edit a profile in the default editor
    Edit {
        /// Name of the profile to edit
        #[clap(short, long)]
        name: String,
    },
}

/// "backup" subcommand
#[derive(clap::Subcommand)]
pub enum BackupCmd {
    /// Create a new backup
    Create {
        /// Name of the profile to back up
        #[clap(short, long)]
        name: String,
    },
    /// Restore the given backup
    Restore {
        /// Name of the profile containing the backup
        #[clap(short, long)]
        name: String,
        // /// Restore by tag
        // #[clap(short, long, conflicts_with = "latest")]
        // tag: Option<String>,
        /// Restore the latest backup
        #[clap(short, long)]
        id: Option<Id>,
    },
    /// List all backups for the given profile
    List {
        /// Name of the profile to list backups for
        #[clap(short, long)]
        name: String,
        /// Number of backups to list
        #[clap(short, long)]
        count: Option<usize>,
    },
    Delete {
        /// Name of the profile to purge backups for
        #[clap(short, long)]
        name: String,
        /// Name of the profile to purge backups for
        #[clap(short, long, default_value = None)]
        id: Option<Id>,
    },
    /// Retain only the "count" latest backups
    Retain {
        #[clap(short, long)]
        name: String,
        #[clap(short, long)]
        count: usize,
    }
}
