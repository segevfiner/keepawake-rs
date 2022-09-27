// TODO Use cfg_if

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::*;

#[cfg(not(windows))]
compile_error!("Unsupported cfg");
