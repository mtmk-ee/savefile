use std::{
    sync::mpsc::{RecvTimeoutError, Sender},
    time::Duration,
};

use notify::{Event, ReadDirectoryChangesWatcher, RecursiveMode};

use crate::{
    backup::backup,
    database::Database,
    error::{ProfileError, Result},
    profile::Profile,
};

pub type Watcher = ReadDirectoryChangesWatcher;

pub fn watch(db: &Database, profile: &Profile, name: &str) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let _watcher = create_watcher(profile, tx)?;
    let mut changed = false;
    loop {
        let timeout = Duration::from_secs_f32(profile.delay());
        match rx.recv_timeout(timeout) {
            Ok(_) => {
                // don't care about which files changed or why,
                // since when we time out we'll change everything
                changed = true;
            }
            Err(RecvTimeoutError::Timeout) => {
                if !changed {
                    continue;
                }
                changed = false;
                println!("--------------------------------------------------");
                println!("{:?}: contents changed on disk", name);
                backup(&db, profile, name)?;
            }
            Err(RecvTimeoutError::Disconnected) => {
                panic!("what! impossible!")
            }
        }
    }
}

fn create_watcher(profile: &Profile, tx: Sender<()>) -> Result<Watcher> {
    use notify::Watcher;
    let idkbro = profile.clone();
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
        let include = idkbro.expand_includes(false).expect("invalid profile");
        if let Ok(event) = res {
            if event.paths.iter().any(|path| include.contains(path)) {
                tx.send(()).expect("failed to send event")
            } else {
                println!("ignoring event: {:?}", event);
            }
        }
    })
    .expect("failed to create watcher");
    watcher
        .watch(&profile.base(), RecursiveMode::Recursive)
        .or(Err(ProfileError::InvalidBase(profile.base().to_owned())))?;
    Ok(watcher)
}
