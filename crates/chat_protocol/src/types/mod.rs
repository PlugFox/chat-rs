//! Protocol types — enums, bitflags, and data structures.
//!
//! All types here are wire-format-aware: their `repr` matches the binary encoding.

mod chat;
mod error;
mod frame;
mod message;
mod user;

pub use chat::*;
pub use error::*;
pub use frame::*;
pub use message::*;
pub use user::*;
