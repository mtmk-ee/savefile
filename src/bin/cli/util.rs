use std::path::Path;

/// Prompts the user to confirm an action.
///
/// Returns `true` if the user confirms, `false` otherwise.
pub fn confirm(msg: &str) -> bool {
    use dialoguer::Confirm;

    Confirm::new()
        .with_prompt(msg)
        .interact()
        .unwrap()
}

/// Returns the path as a string, with backslashes replaced with forward slashes.
pub fn path_str(path: impl AsRef<Path>) -> String {
    path.as_ref().display().to_string().replace("\\", "/")
}
