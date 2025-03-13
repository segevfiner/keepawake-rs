use crate::Options;

pub type Error = jni::errors::Error;

pub struct KeepAwake {
    options: Options,
}

impl KeepAwake {
    pub fn new(options: Options) -> Result<Self, Error> {
        let mut awake = Self {
            options,
        };
        Ok(awake)
    }
}

impl Drop for KeepAwake {
    fn drop(&mut self) {

    }
}
