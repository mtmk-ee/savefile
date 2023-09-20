use std::path::{Path, PathBuf};

use crate::{
    error::{Error, ProfileError, Result},
    filesystem::profiles_dir,
};

/// A profile is primarily a specification of which files to back up.
///
/// Files to back up are specified as glob patterns relative to the profile's base directory.
/// The `delay` field specifies the time to wait after a save file is modified before backing
/// up everything.
///
/// The profile is stored as a JSON file in the profiles directory.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Profile {
    /// Root directory which includes are relative to
    base: PathBuf,
    /// Glob patterns for files to watch/include in the backup
    include: Vec<String>,
    /// The time to wait after a save file is modified before backing up everything.
    delay: f32,
}

impl Profile {
    /// Create a new profile with the given base directory.
    ///
    /// Defaults:
    /// - `include`: `[]`
    /// - `delay`: `5.0`
    pub fn new<P: AsRef<Path>>(base: P) -> Self {
        Self {
            base: base.as_ref().to_owned(),
            include: Vec::new(),
            delay: 5f32,
        }
    }

    /// Open a profile from the given path.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_owned();
        let contents =
            std::fs::read(&path).or_else(|_| Err(ProfileError::NoSuchProfile(path.clone())))?;
        let profile = serde_json::from_slice(&contents)
            .or_else(|_| Err(ProfileError::InvalidFormat(path)))?;
        Ok(profile)
    }

    /// Returns the path to the target base directory.
    pub fn base(&self) -> &Path {
        &self.base
    }

    /// Returns the glob patterns for files to watch/include in the backup.
    pub fn includes(&self) -> &[String] {
        &self.include
    }

    /// Returns the time to wait after a save file is modified before backing up everything.
    pub fn delay(&self) -> f32 {
        self.delay
    }

    /// Save the profile to the given path.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let ser = serde_json::to_string_pretty(self)
            .or(Err(ProfileError::InvalidFormat(path.as_ref().to_owned())))?;
        std::fs::write(path, ser)?;
        Ok(())
    }

    /// Expand the glob patterns in `includes()`.
    ///
    /// Returned paths may either be absolute or relative to `base()`.
    pub fn expand_includes(&self, relative: bool) -> Result<Vec<PathBuf>> {
        let mut paths = self
            .includes()
            .iter()
            .flat_map(|glob| {
                glob::glob(&format!("{}/{}", self.base().display(), glob)).expect("invalid glob")
            })
            .filter_map(|res| res.ok())
            .map(|path| {
                if relative {
                    path.strip_prefix(self.base())
                        .expect("invalid profile")
                        .to_owned()
                } else {
                    path
                }
            })
            .collect::<Vec<_>>();

        // remove duplicate paths
        paths.sort();
        paths.dedup();

        Ok(paths)
    }
}

/// List all profiles in the profiles directory.
pub fn list_profiles() -> Result<Vec<(PathBuf, Profile)>> {
    let profiles_dir = profiles_dir()?;
    let profiles = std::fs::read_dir(profiles_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|entry| entry.is_file())
        .map(|entry| {
            let profile = Profile::open(&entry)?;
            Ok((entry, profile))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(profiles)
}
