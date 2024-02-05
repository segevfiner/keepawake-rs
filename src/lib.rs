//! Keep your computer awake.
//!
//! # Examples
//!
//! ```
//! # fn try_main() -> Result<(), keepawake::Error> {
//! let _awake = keepawake::Builder::default()
//!     .display(true)
//!     .reason("Video playback")
//!     .app_name("My prog")
//!     .app_reverse_domain("io.github.myprog")
//!     .create()?;
//! # Ok(())
//! # }
//! # try_main();
//! ```
//!
//! ```
//! # fn try_main() -> Result<(), keepawake::Error> {
//! let _awake = keepawake::Builder::default()
//!     .display(true)
//!     .idle(true)
//!     .sleep(true)
//!     .create()?;
//! # Ok(())
//! # }
//! # try_main();
//! ```

use derive_builder::Builder;
use thiserror::Error;

mod sys;


#[cfg(feature = "capi")]
pub mod capi;

#[derive(Error, Debug)]
pub enum Error {
    #[error("builder: {0}")]
    Builder(#[from] BuilderError),

    #[error("system: {0}")]
    System(#[from] sys::Error),
}

#[derive(Builder, Debug)]
#[builder(public, name = "Builder", build_fn(private))]
#[allow(dead_code)] // Some fields are unused on some platforms
struct Options {
    /// Prevent the display from turning off.
    #[builder(default)]
    display: bool,

    /// Prevent the system from sleeping due to idleness.
    #[builder(default)]
    idle: bool,

    /// Prevent the system from sleeping. Only works under certain, OS dependant, conditions.
    #[builder(default)]
    sleep: bool,

    // TODO Reconsider this defaults. They are really meant for the CLI.
    /// Reason the consumer is keeping the system awake. Defaults to `"User requested"`. (Used on Linux & macOS)
    #[builder(setter(into), default = "\"User requested\".to_string()")]
    reason: String,

    /// Name of the program keeping the system awake. Defaults to `"keepawake-rs"`. (Used on Linux)
    #[builder(setter(into), default = "\"keepawake-rs\".to_string()")]
    app_name: String,

    /// Reverse domain name of the program keeping the system awake. Defaults to `"io.github.segevfiner.keepawake-rs"`. (Used on Linux)
    #[builder(
        setter(into),
        default = "\"io.github.segevfiner.keepawake-rs\".to_string()"
    )]
    app_reverse_domain: String,
}

impl Builder {
    /// Create the [`KeepAwake`].
    pub fn create(&self) -> Result<KeepAwake, Error> {
        Ok(KeepAwake {
            _imp: sys::KeepAwake::new(self.build()?)?,
        })
    }
}

/// Keeps the machine or display awake (as configured), until dropped. Create using [struct@Builder].
pub struct KeepAwake {
    _imp: sys::KeepAwake,
}
