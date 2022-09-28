//! Using [`org.freedesktop.ScreenSaver`], and [systemd Inhibitor Locks].
//!
//! For VT, should we add support for using `setterm -blank`? is there something better?
//!
//! Debug with `systemd-inhibit --list`, `gnome-session-inhibit --list`.
//!
//! [ScreenSaver]: https://people.freedesktop.org/~hadess/idle-inhibition-spec/re01.html
//! [systemd Inhibitor Locks]:(https://www.freedesktop.org/wiki/Software/systemd/inhibit/

use std::{error::Error, time::Duration};

use dbus::blocking::{Connection, Proxy};

use crate::AwakeOptions;

pub struct Awake {
    options: AwakeOptions,

    session_conn: Option<Connection>,
    cookie: Option<u32>,

    system_conn: Option<Connection>,
    idle_fd: Option<dbus::arg::OwnedFd>,
    sleep_fd: Option<dbus::arg::OwnedFd>,
}

impl Awake {
    pub fn new(options: &AwakeOptions) -> Result<Self, Box<dyn Error>> {
        let mut awake = Awake {
            options: *options,
            cookie: None,
            session_conn: None,
            system_conn: None,
            idle_fd: None,
            sleep_fd: None,
        };
        awake.set()?;
        Ok(awake)
    }

    fn set(&mut self) -> Result<(), Box<dyn Error>> {
        self.cookie = if self.options.display {
            self.session_conn = Some(Connection::new_session()?);
            let result: (u32,) = self.screensaver_proxy().method_call(
                "org.freedesktop.ScreenSaver",
                "Inhibit",
                ("io.github.segevfiner.keepawake-rs", "User requested"),
            )?;
            Some(result.0)
        } else {
            None
        };

        self.idle_fd = if self.options.idle {
            self.system_conn = Some(Connection::new_system()?);
            let result: (dbus::arg::OwnedFd,) = self.login_proxy().method_call(
                "org.freedesktop.login1.Manager",
                "Inhibit",
                ("idle", "keepawake-rs", "User requested", "block"),
            )?;
            Some(result.0)
        } else {
            None
        };

        self.sleep_fd = if self.options.sleep {
            self.system_conn = Some(Connection::new_system()?);
            let result: (dbus::arg::OwnedFd,) = self.login_proxy().method_call(
                "org.freedesktop.login1.Manager",
                "Inhibit",
                ("sleep", "keepawake-rs", "User requested", "block"),
            )?;
            Some(result.0)
        } else {
            None
        };

        Ok(())
    }

    fn screensaver_proxy(&self) -> Proxy<&Connection> {
        self.session_conn.as_ref().unwrap().with_proxy(
            "org.freedesktop.ScreenSaver",
            "/org/freedesktop/ScreenSaver",
            Duration::from_millis(5000),
        )
    }

    fn login_proxy(&self) -> Proxy<&Connection> {
        self.system_conn.as_ref().unwrap().with_proxy(
            "org.freedesktop.login1",
            "/org/freedesktop/login1",
            Duration::from_millis(5000),
        )
    }
}

impl Drop for Awake {
    fn drop(&mut self) {
        #[allow(clippy::let_unit_value)]
        if self.options.display {
            () = self
                .screensaver_proxy()
                .method_call(
                    "org.freedesktop.ScreenSaver",
                    "UnInhibit",
                    (self.cookie.unwrap(),),
                )
                .unwrap();
        }
    }
}
