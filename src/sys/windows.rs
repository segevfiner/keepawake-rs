use std::error::Error;

use windows::Win32::System::Power::{
    SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
    EXECUTION_STATE,
};
use windows::core::Error as WindowsError;

use crate::AwakeOptions;

pub struct Awake {
    options: AwakeOptions,
    previous: EXECUTION_STATE,
}

impl Awake {
    pub fn new(options: &AwakeOptions) -> Result<Self, Box<dyn Error>> {
        let mut this = Awake {
            options: *options,
            previous: Default::default(),
        };
        this.set()?;
        Ok(this)
    }

    fn set(&mut self) -> Result<(), WindowsError> {
        let mut esflags = ES_CONTINUOUS;

        if self.options.display {
            esflags |= ES_DISPLAY_REQUIRED;
        }

        if self.options.idle {
            esflags |= ES_SYSTEM_REQUIRED;
        }

        unsafe {
            self.previous = SetThreadExecutionState(esflags);
            if self.previous == EXECUTION_STATE(0) {
                return Err(WindowsError::from_win32());
            }

            Ok(())
        }
    }
}

impl Drop for Awake {
    fn drop(&mut self) {
        unsafe {
            SetThreadExecutionState(self.previous);
        }
    }
}
