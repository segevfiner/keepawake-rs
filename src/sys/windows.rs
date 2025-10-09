//! Using [`SetThreadExecutionState`], can alternatively use [`PowerSetRequest`].
//!
//! Using away mode to prevent explicit sleep seems to be unsupported with modern standby.
//!
//! Debug with `powercfg /requests`.
//!
//! [`SetThreadExecutionState`]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadexecutionstate
//! [`PowerSetRequest`]: https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-powersetrequest

use windows::core::Error as WindowsError;
use windows::Win32::System::Power::{
    SetThreadExecutionState, ES_AWAYMODE_REQUIRED, ES_CONTINUOUS, ES_DISPLAY_REQUIRED,
    ES_SYSTEM_REQUIRED, EXECUTION_STATE,
};

use crate::Options;

pub type Error = WindowsError;

pub struct KeepAwake {
    options: Options,
    previous: EXECUTION_STATE,
}

impl KeepAwake {
    pub fn new(options: Options) -> Result<Self, Error> {
        let mut awake = KeepAwake {
            options,
            previous: Default::default(),
        };
        awake.set()?;
        Ok(awake)
    }

    fn set(&mut self) -> Result<(), Error> {
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
                return Err(WindowsError::from_thread());
            }

            Ok(())
        }
    }
}

impl Drop for KeepAwake {
    fn drop(&mut self) {
        unsafe {
            SetThreadExecutionState(self.previous);
        }
    }
}
