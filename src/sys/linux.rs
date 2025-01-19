//! Using [`org.freedesktop.ScreenSaver`], and [systemd Inhibitor Locks].
//!
//! For VT, should we add support for using `setterm -blank`? is there something better?
//!
//! Debug with `systemd-inhibit --list`, `gnome-session-inhibit --list`.
//!
//! [ScreenSaver]: https://people.freedesktop.org/~hadess/idle-inhibition-spec/re01.html
//! [systemd Inhibitor Locks]:(https://www.freedesktop.org/wiki/Software/systemd/inhibit/

use zbus::{blocking::Connection, proxy};

use crate::Options;

pub type Error = zbus::Error;

#[proxy(
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

#[proxy(assume_defaults = true)]
trait ScreenSaver {
    /// Inhibit method
    fn inhibit(&self, application_name: &str, reason_for_inhibit: &str) -> zbus::Result<u32>;

    /// UnInhibit method
    fn un_inhibit(&self, cookie: u32) -> zbus::Result<()>;
}

pub struct KeepAwake {
    options: Options,

    session_conn: Option<Connection>,
    screensaver_proxy: Option<ScreenSaverProxyBlocking<'static>>,
    cookie: Option<u32>,

    system_conn: Option<Connection>,
    manager_proxy: Option<ManagerProxyBlocking<'static>>,
    idle_fd: Option<zbus::zvariant::OwnedFd>,
    sleep_fd: Option<zbus::zvariant::OwnedFd>,
}

impl KeepAwake {
    pub fn new(options: Options) -> Result<Self, zbus::Error> {
        let mut awake = Self {
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
            Some(
                self.screensaver_proxy
                    .as_ref()
                    .unwrap()
                    .inhibit(&self.options.app_reverse_domain, &self.options.reason)?,
            )
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
                &self.options.app_name,
                &self.options.reason,
                "block",
            )?)
        } else {
            None
        };

        self.sleep_fd = if self.options.sleep {
            Some(self.manager_proxy.as_ref().unwrap().inhibit(
                "sleep",
                &self.options.app_name,
                &self.options.reason,
                "block",
            )?)
        } else {
            None
        };

        Ok(())
    }
}

impl Drop for KeepAwake {
    fn drop(&mut self) {
        if self.options.display {
            if let (Some(p), Some(cookie)) = (self.screensaver_proxy.as_ref(), self.cookie) {
                p.un_inhibit(cookie).unwrap()
            }
        }
    }
}
