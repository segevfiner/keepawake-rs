//! Using [`IOPMAssertionCreateWithName`].
//!
//! Debug with `pmset -g assertions`.
//!
//! [`IOPMAssertionCreateWithName`]: https://developer.apple.com/documentation/iokit/1557134-iopmassertioncreatewithname

use objc2_core_foundation::CFString;
use objc2_io_kit::{
    kIOPMAssertionLevelOn, kIOReturnSuccess, IOPMAssertionCreateWithName, IOPMAssertionID,
    IOPMAssertionRelease,
};
use thiserror::Error;

use crate::Options;

#[derive(Error, Debug)]
#[error("IO error: {code:#010x}")]
pub struct IOError {
    code: i32,
}

pub type Error = IOError;

#[allow(non_upper_case_globals)]
const kIOPMAssertionTypePreventUserIdleSystemSleep: &str = "PreventUserIdleSystemSleep";

#[allow(non_upper_case_globals)]
const kIOPMAssertionTypePreventUserIdleDisplaySleep: &str = "PreventUserIdleDisplaySleep";

#[allow(non_upper_case_globals)]
const kIOPMAssertionTypePreventSystemSleep: &str = "PreventSystemSleep";

pub struct KeepAwake {
    options: Options,

    display_assertion: IOPMAssertionID,
    idle_assertion: IOPMAssertionID,
    sleep_assertion: IOPMAssertionID,
}

impl KeepAwake {
    pub fn new(options: Options) -> Result<Self, Error> {
        let mut awake = Self {
            options,
            display_assertion: 0,
            idle_assertion: 0,
            sleep_assertion: 0,
        };
        awake.set()?;
        Ok(awake)
    }

    fn set(&mut self) -> Result<(), Error> {
        if self.options.display {
            unsafe {
                let assertion_type =
                    CFString::from_static_str(kIOPMAssertionTypePreventUserIdleDisplaySleep);
                let assertion_name = CFString::from_str(&self.options.reason);
                let result = IOPMAssertionCreateWithName(
                    Some(&assertion_type),
                    kIOPMAssertionLevelOn,
                    Some(&assertion_name),
                    &mut self.display_assertion,
                );
                if result != kIOReturnSuccess {
                    return Err(Error { code: result });
                }
            }
        }

        if self.options.idle {
            unsafe {
                let assertion_type =
                    CFString::from_static_str(kIOPMAssertionTypePreventUserIdleSystemSleep);
                let assertion_name = CFString::from_str(&self.options.reason);
                let result = IOPMAssertionCreateWithName(
                    Some(&assertion_type),
                    kIOPMAssertionLevelOn,
                    Some(&assertion_name),
                    &mut self.idle_assertion,
                );
                if result != kIOReturnSuccess {
                    return Err(Error { code: result });
                }
            }
        }

        if self.options.sleep {
            unsafe {
                let assertion_type =
                    CFString::from_static_str(kIOPMAssertionTypePreventSystemSleep);
                let assertion_name = CFString::from_str(&self.options.reason);
                let result = IOPMAssertionCreateWithName(
                    Some(&assertion_type),
                    kIOPMAssertionLevelOn,
                    Some(&assertion_name),
                    &mut self.sleep_assertion,
                );
                if result != kIOReturnSuccess {
                    return Err(Error { code: result });
                }
            }
        }

        Ok(())
    }
}

impl Drop for KeepAwake {
    fn drop(&mut self) {
        if self.display_assertion != 0 {
            IOPMAssertionRelease(self.display_assertion);
        }

        if self.idle_assertion != 0 {
            IOPMAssertionRelease(self.idle_assertion);
        }

        if self.sleep_assertion != 0 {
            IOPMAssertionRelease(self.sleep_assertion);
        }
    }
}
