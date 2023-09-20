use clap::Parser;
use cli::args::{Args, BackupCmd, ProfileCmd, SubCmd};
use savefile::{error::Result, filesystem::create_required_dirs};

mod cli;

fn main() {
    create_required_dirs().expect("failed to create required directories");
    let res = match Args::parse().cmd {
        SubCmd::Profile(cmd) => profile_cmd(cmd),
        SubCmd::Watch { name } => cli::run_watcher(&name),
        SubCmd::Backup(cmd) => backup_cmd(cmd),
    };
    if let Err(err) = res {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

/// Handle the "profile" subcommand.
pub fn profile_cmd(cmd: ProfileCmd) -> Result<()> {
    match cmd {
        ProfileCmd::List { prefix } => cli::print_profiles(prefix),
        ProfileCmd::Browse => cli::open_profiles_dir(),
        ProfileCmd::Edit { name } => cli::edit_profile(&name),
        ProfileCmd::Create { name, edit } => cli::create_profile(&name, edit),
        ProfileCmd::Delete { name } => cli::delete_profile(&name),
    }
}

/// Handle the "backup" subcommand.
pub fn backup_cmd(cmd: BackupCmd) -> Result<()> {
    match cmd {
        BackupCmd::Create { name } => cli::create_backup(&name),
        BackupCmd::Delete { name, id } => cli::delete_backup(&name, id),
        BackupCmd::List { name, count } => cli::print_backups(&name, count),
        BackupCmd::Restore { name, id } => cli::restore_backup(&name, id),
        BackupCmd::Retain { name, count } => cli::retain_backups(&name, count),
    }
}
