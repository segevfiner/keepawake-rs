//! Using [`org.freedesktop.ScreenSaver`], and [systemd Inhibitor Locks].
//!
//! For VT, should we add support for using `setterm -blank`? is there something better?
//!
//! Debug with `systemd-inhibit --list`, `gnome-session-inhibit --list`.
//!
//! [ScreenSaver]: https://people.freedesktop.org/~hadess/idle-inhibition-spec/re01.html
//! [systemd Inhibitor Locks]:(https://www.freedesktop.org/wiki/Software/systemd/inhibit/

use anyhow::Result;
use zbus::{blocking::Connection, dbus_proxy};

use crate::Builder;

#[dbus_proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait Manager {
    /// Inhibit method
    fn inhibit(
        &self,
        what: &str,
        who: &str,
        why: &str,
        mode: &str,
    ) -> zbus::Result<zbus::zvariant::OwnedFd>;
}

#[dbus_proxy(assume_defaults = true)]
trait ScreenSaver {
    /// Inhibit method
    fn inhibit(&self, application_name: &str, reason_for_inhibit: &str) -> zbus::Result<u32>;

    /// UnInhibit method
    fn un_inhibit(&self, cookie: u32) -> zbus::Result<()>;
}

pub struct Awake {
    options: Builder,

    session_conn: Option<Connection>,
    screensaver_proxy: Option<ScreenSaverProxyBlocking<'static>>,
    cookie: Option<u32>,

    system_conn: Option<Connection>,
    manager_proxy: Option<ManagerProxyBlocking<'static>>,
    idle_fd: Option<zbus::zvariant::OwnedFd>,
    sleep_fd: Option<zbus::zvariant::OwnedFd>,
}

impl Awake {
    pub fn new(options: Builder) -> Result<Self> {
        let mut awake = Awake {
            options,

            session_conn: None,
            screensaver_proxy: None,
            cookie: None,

            system_conn: None,
            manager_proxy: None,
            idle_fd: None,
            sleep_fd: None,
        };
        awake.set()?;
        Ok(awake)
    }

    fn set(&mut self) -> Result<(), zbus::Error> {
        self.cookie = if self.options.display {
            self.session_conn = Some(Connection::session()?);
            self.screensaver_proxy = Some(ScreenSaverProxyBlocking::new(
                self.session_conn.as_ref().unwrap(),
            )?);
            Some(self.screensaver_proxy.as_ref().unwrap().inhibit(
                self.options.app_reverse_domain_or_default(),
                self.options.reason_or_default(),
            )?)
        } else {
            None
        };

        if self.options.idle || self.options.sleep {
            self.system_conn = Some(Connection::system()?);
            self.manager_proxy = Some(ManagerProxyBlocking::new(
                self.system_conn.as_ref().unwrap(),
            )?);
        }

        self.idle_fd = if self.options.idle {
            Some(self.manager_proxy.as_ref().unwrap().inhibit(
                "idle",
                self.options.app_name_or_default(),
                self.options.reason_or_default(),
                "block",
            )?)
        } else {
            None
        };

        self.sleep_fd = if self.options.sleep {
            Some(self.manager_proxy.as_ref().unwrap().inhibit(
                "sleep",
                self.options.app_name_or_default(),
                self.options.reason_or_default(),
                "block",
            )?)
        } else {
            None
        };

        Ok(())
    }
}

impl Drop for Awake {
    fn drop(&mut self) {
        if self.options.display {
            if let (Some(p), Some(cookie)) = (self.screensaver_proxy.as_ref(), self.cookie) {
                p.un_inhibit(cookie).unwrap()
            }
        }
    }
}
