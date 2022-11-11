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
    /// Reason name of the program keeping the system awake. Defaults to "keepawake-rs" if not given.
    pub consumer: Option<String>,
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
