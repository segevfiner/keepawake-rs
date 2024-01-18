//! Keep your computer awake.
//!
//! # Examples
//!
//! ```
//! # fn try_main() -> anyhow::Result<()> {
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
//! //!
//! ```
//! # fn try_main() -> anyhow::Result<()> {
//! let _awake = keepawake::Builder::default()
//!     .display(true)
//!     .idle(true)
//!     .sleep(true)
//!     .create()?;
//! # Ok(())
//! # }
//! # try_main();
//! ```

use anyhow::Result;
use derive_builder::Builder;

mod sys;

#[derive(Builder, Debug, Default)]
#[builder(public, default, name = "Builder", build_fn(private))]
#[allow(dead_code)] // Some fields are unused on some platforms
struct Options {
    display: bool,
    idle: bool,
    sleep: bool,

    // TODO Reconsider this defaults. They are really meant for the CLI.
    #[builder(setter(into), default = "\"User requested\".to_string()")]
    reason: String,

    #[builder(setter(into), default = "\"keepawake-rs\".to_string()")]
    app_name: String,

    #[builder(setter(into), default = "\"io.github.segevfiner.keepawake-rs\".to_string()")]
    app_reverse_domain: String,
}

impl Builder {
    pub fn create(&self) -> Result<KeepAwake> {
        Ok(KeepAwake {
            _imp: sys::KeepAwake::new(self.build()?)?,
        })
    }
}

/// Keeps the machine or display awake (as configured), until dropped. Create using [Builder].
pub struct KeepAwake {
    _imp: sys::KeepAwake,
}
