use outcome::Outcome;

/// Declares a simple interfaces, that all notification backends must implement.
pub trait Notify {
    /// Notify a system user about test outcome.
    /// This must result into showing a notification message on user's desktop.
    fn notify(&self, outcome: Outcome);
}

mod notify_send;
pub use self::notify_send::NotifySend;
