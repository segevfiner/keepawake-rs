use std::error::Error;

mod sys;

#[derive(Clone, Copy, Debug, Default)]
pub struct AwakeOptions {
    pub display: bool,
    pub idle: bool,
}

pub struct Awake {
    _imp: sys::Awake,
    _options: AwakeOptions,
}

impl Awake {
    pub fn new(options: &AwakeOptions) -> Result<Self, Box<dyn Error>> {
        Ok(Awake {
            _imp: sys::Awake::new(options)?,
            _options: *options,
        })
    }
}
