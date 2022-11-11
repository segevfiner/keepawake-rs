use anyhow::Result;

mod sys;

// TODO Should this be a builder for Awake instead of public fields?
#[derive(Clone, Debug, Default)]
pub struct AwakeOptions {
    pub display: bool,
    pub idle: bool,
    pub sleep: bool,
    /// Reason the consumer is keeping the system awake. Defaults to "User requested".
    pub reason: Option<String>,
    /// Name of the program keeping the system awake. Defaults to "keepawake-rs" if not given.
    pub consumer: Option<String>,
    /// Domain name of the program keeping the system awake. Defaults to "io.github.segevfiner.keepawake-rs" if not given.
    pub consumer_domain: Option<String>,
}

impl AwakeOptions {
    #[allow(dead_code)] // unused on Windows
    pub(crate) fn reason(&self) -> &str {
        self.reason.as_deref().unwrap_or("User requested")
    }

    #[allow(dead_code)] // unused on macOS and Windows
    pub(crate) fn consumer(&self) -> &str {
        self.reason.as_deref().unwrap_or("keepawake-rs")
    }

    #[allow(dead_code)] // unused on macOS and Windows
    pub(crate) fn consumer_domain(&self) -> &str {
        self.reason.as_deref().unwrap_or("io.github.segevfiner.keepawake-rs")
    }
}

/// Once created, keeps the machine or display awake (as requested in the
/// AwakeOptions) until dropped.
pub struct Awake {
    _imp: sys::Awake,
}

impl Awake {
    pub fn new(options: AwakeOptions) -> Result<Self> {
        Ok(Awake {
            _imp: sys::Awake::new(options)?,
        })
    }
}
