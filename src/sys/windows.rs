//! Using [`SetThreadExecutionState`], can alternatively use [`PowerSetRequest`].
//!
//! Using away mode to prevent explicit sleep seems to be unsupported with modern standby.
//!
//! Debug with `powercfg /requests`.
//!
//! [`SetThreadExecutionState`]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadexecutionstate
//! [`PowerSetRequest`]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-powersetrequest

use anyhow::Result;
use windows::core::Error as WindowsError;
use windows::Win32::System::Power::{
    SetThreadExecutionState, ES_AWAYMODE_REQUIRED, ES_CONTINUOUS, ES_DISPLAY_REQUIRED,
    ES_SYSTEM_REQUIRED, EXECUTION_STATE,
};

use crate::Builder;

pub struct Awake {
    options: Builder,
    previous: EXECUTION_STATE,
}

impl Awake {
    pub fn new(options: Builder) -> Result<Self> {
        let mut awake = Awake {
            options,
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

        if self.options.sleep {
            esflags |= ES_AWAYMODE_REQUIRED;
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
