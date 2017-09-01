use report::Report;

/// Declares a simple interfaces, that all notification backends must implement.
pub trait Notify {
    /// Notify a system user about test outcome.
    /// This must result into showing a notification message on user's desktop.
    fn notify(&self, report: Report);
}

mod notify_send;
pub use self::notify_send::NotifySend;

mod osascript;
pub use self::osascript::Osascript;
