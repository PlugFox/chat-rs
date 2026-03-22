//! Low-level wire format helpers — read/write primitives.
//!
//! All multi-byte integers are little-endian.

use crate::MIN_TIMESTAMP;
use crate::error::CodecError;
use bytes::{Buf, BufMut};

/// Ensure `buf` has at least `needed` bytes remaining.
#[inline]
pub fn ensure_remaining(buf: &impl Buf, needed: usize) -> Result<(), CodecError> {
    let available = buf.remaining();
    if available < needed {
        return Err(CodecError::Truncated { needed, available });
    }
    Ok(())
}

// --- Read primitives ---

/// Read a `u8`.
#[inline]
pub fn read_u8(buf: &mut impl Buf) -> Result<u8, CodecError> {
    ensure_remaining(buf, 1)?;
    Ok(buf.get_u8())
}

/// Read a `u16` (little-endian).
#[inline]
pub fn read_u16(buf: &mut impl Buf) -> Result<u16, CodecError> {
    ensure_remaining(buf, 2)?;
    Ok(buf.get_u16_le())
}

/// Read a `u32` (little-endian).
#[inline]
pub fn read_u32(buf: &mut impl Buf) -> Result<u32, CodecError> {
    ensure_remaining(buf, 4)?;
    Ok(buf.get_u32_le())
}

/// Read an `i64` (little-endian).
#[inline]
pub fn read_i64(buf: &mut impl Buf) -> Result<i64, CodecError> {
    ensure_remaining(buf, 8)?;
    Ok(buf.get_i64_le())
}

/// Read an `i64` timestamp and validate it is within the allowed range.
///
/// Valid range: `0 ≤ value < 2⁴¹` (see `docs/codec.md`).
/// Fast check: `value >> 41 != 0` → reject.
#[inline]
pub fn read_timestamp(buf: &mut impl Buf) -> Result<i64, CodecError> {
    let value = read_i64(buf)?;
    validate_timestamp(value)?;
    Ok(value)
}

/// Validate a timestamp is within the allowed range.
#[inline]
pub fn validate_timestamp(value: i64) -> Result<(), CodecError> {
    if value < MIN_TIMESTAMP || value >> 41 != 0 {
        return Err(CodecError::TimestampOutOfRange(value));
    }
    Ok(())
}

/// Read a length-prefixed UTF-8 string (`u32 len` + bytes).
///
/// Returns an empty string when `len = 0` (no allocation for the string data).
pub fn read_string(buf: &mut impl Buf) -> Result<String, CodecError> {
    let len = read_u32(buf)? as usize;
    if len == 0 {
        return Ok(String::new());
    }
    ensure_remaining(buf, len)?;
    let bytes = read_bytes_exact(buf, len);
    String::from_utf8(bytes).map_err(|e| CodecError::InvalidUtf8 {
        offset: e.utf8_error().valid_up_to(),
    })
}

/// Read an optional length-prefixed UTF-8 string.
///
/// Returns `None` when `len = 0` (absent field).
pub fn read_optional_string(buf: &mut impl Buf) -> Result<Option<String>, CodecError> {
    let len = read_u32(buf)? as usize;
    if len == 0 {
        return Ok(None);
    }
    ensure_remaining(buf, len)?;
    let bytes = read_bytes_exact(buf, len);
    let s = String::from_utf8(bytes).map_err(|e| CodecError::InvalidUtf8 {
        offset: e.utf8_error().valid_up_to(),
    })?;
    Ok(Some(s))
}

/// Read a length-prefixed byte blob (`u32 len` + bytes).
///
/// Returns `None` when `len = 0`.
pub fn read_optional_bytes(buf: &mut impl Buf) -> Result<Option<Vec<u8>>, CodecError> {
    let len = read_u32(buf)? as usize;
    if len == 0 {
        return Ok(None);
    }
    ensure_remaining(buf, len)?;
    Ok(Some(read_bytes_exact(buf, len)))
}

/// Read a UUID (16 bytes, raw).
pub fn read_uuid(buf: &mut impl Buf) -> Result<uuid::Uuid, CodecError> {
    ensure_remaining(buf, 16)?;
    let mut bytes = [0u8; 16];
    buf.copy_to_slice(&mut bytes);
    Ok(uuid::Uuid::from_bytes(bytes))
}

/// Read an `Option<u32>` encoded as `u8 flag` + optional `u32`.
///
/// Flag `0` = absent, flag `1` = value follows.
pub fn read_option_u32(buf: &mut impl Buf) -> Result<Option<u32>, CodecError> {
    let flag = read_u8(buf)?;
    match flag {
        0 => Ok(None),
        1 => Ok(Some(read_u32(buf)?)),
        _ => Err(CodecError::UnknownDiscriminant {
            type_name: "Option<u32>",
            value: flag as u32,
        }),
    }
}

/// Read exactly `len` bytes from `buf` into a new `Vec<u8>`.
fn read_bytes_exact(buf: &mut impl Buf, len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    buf.copy_to_slice(&mut out);
    out
}

// --- Write primitives ---

/// Write a `u8`.
#[inline]
pub fn write_u8(buf: &mut impl BufMut, value: u8) {
    buf.put_u8(value);
}

/// Write a `u16` (little-endian).
#[inline]
pub fn write_u16(buf: &mut impl BufMut, value: u16) {
    buf.put_u16_le(value);
}

/// Write a `u32` (little-endian).
#[inline]
pub fn write_u32(buf: &mut impl BufMut, value: u32) {
    buf.put_u32_le(value);
}

/// Write an `i64` (little-endian).
#[inline]
pub fn write_i64(buf: &mut impl BufMut, value: i64) {
    buf.put_i64_le(value);
}

/// Write an `i64` timestamp after validation.
#[inline]
pub fn write_timestamp(buf: &mut impl BufMut, value: i64) -> Result<(), CodecError> {
    validate_timestamp(value)?;
    write_i64(buf, value);
    Ok(())
}

/// Write a length-prefixed UTF-8 string (`u32 len` + bytes).
///
/// Writes `len = 0` for an empty string.
#[inline]
pub fn write_string(buf: &mut impl BufMut, s: &str) {
    write_u32(buf, s.len() as u32);
    if !s.is_empty() {
        buf.put_slice(s.as_bytes());
    }
}

/// Write an optional length-prefixed UTF-8 string.
///
/// Writes `len = 0` for `None`.
#[inline]
pub fn write_optional_string(buf: &mut impl BufMut, s: Option<&str>) {
    match s {
        Some(s) => write_string(buf, s),
        None => write_u32(buf, 0),
    }
}

/// Write an optional length-prefixed byte blob.
///
/// Writes `len = 0` for `None`.
#[inline]
pub fn write_optional_bytes(buf: &mut impl BufMut, data: Option<&[u8]>) {
    match data {
        Some(data) => {
            write_u32(buf, data.len() as u32);
            buf.put_slice(data);
        }
        None => write_u32(buf, 0),
    }
}

/// Write a UUID (16 bytes, raw).
#[inline]
pub fn write_uuid(buf: &mut impl BufMut, uuid: &uuid::Uuid) {
    buf.put_slice(uuid.as_bytes());
}

/// Write an `Option<u32>` as `u8 flag` + optional `u32`.
#[inline]
pub fn write_option_u32(buf: &mut impl BufMut, value: Option<u32>) {
    match value {
        Some(v) => {
            write_u8(buf, 1);
            write_u32(buf, v);
        }
        None => write_u8(buf, 0),
    }
}
