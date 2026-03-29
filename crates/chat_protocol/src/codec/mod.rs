//! Binary codec — encode/decode for wire format.
//!
//! All values are little-endian. See `docs/codec.md` for the wire format specification.

mod frame;
mod header;
mod message;
mod payload;
mod wire;

pub use frame::*;
pub use header::*;
pub use message::*;
pub use payload::*;
pub use wire::*;
