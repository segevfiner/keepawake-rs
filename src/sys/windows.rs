//! Using [`SetThreadExecutionState`], can alternatively use [`PowerSetRequest`].
//!
//! Using away mode to prevent explicit sleep seems to be unsupported with modern standby.
//!
//! [`SetThreadExecutionState`]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadexecutionstate
//! [`PowerSetRequest`]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-powersetrequest

use std::error::Error;

use windows::core::Error as WindowsError;
use windows::Win32::System::Power::{
    SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
    EXECUTION_STATE,
};

use crate::AwakeOptions;

pub struct Awake {
    options: AwakeOptions,
    previous: EXECUTION_STATE,
}

impl Awake {
    pub fn new(options: &AwakeOptions) -> Result<Self, Box<dyn Error>> {
        let mut awake = Awake {
            options: *options,
            previous: Default::default(),
        };
        awake.set()?;
        Ok(awake)
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
