use anyhow::Result;

mod sys;

// TODO Should this be a builder for Awake instead of public fields?
#[derive(Clone, Copy, Debug, Default)]
pub struct AwakeOptions {
    pub display: bool,
    pub idle: bool,
    pub sleep: bool,
}

pub struct Awake {
    _imp: sys::Awake,
    _options: AwakeOptions,
}

impl Awake {
    pub fn new(options: &AwakeOptions) -> Result<Self> {
        Ok(Awake {
            _imp: sys::Awake::new(options)?,
            _options: *options,
        })
    }
}
