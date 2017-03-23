//! # All base app elements
//! This is documentation for the `base` module.


pub mod daemon;
pub mod operation;

pub use self::daemon::Daemon;
pub use self::daemon::Status;
pub use self::operation::Operation;
pub use self::operation::DebugPrint;
pub use self::operation::Ls;
pub use self::operation::FakeSpinner;
