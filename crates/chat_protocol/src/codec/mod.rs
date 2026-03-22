//! Binary codec — encode/decode for wire format.
//!
//! All values are little-endian. See `docs/codec.md` for the wire format specification.

mod header;
mod message;
mod payload;
mod wire;

pub use header::*;
pub use message::*;
pub use payload::*;
pub use wire::*;
