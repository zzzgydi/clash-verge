use once_cell::sync::OnceCell;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

static ID: OnceCell<String> = OnceCell::new();

/// This function is meant for use-cases where the default [`prepare`] function can't be used.
///
/// # Errors
/// If ID was already set this functions returns an error containing the ID as String.
#[allow(unused)]
pub fn set_identifier(identifier: &str) -> Result<(), String> {
    ID.set(identifier.to_string())
}
