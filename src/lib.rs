//! `no_std` friendly interface for conversion to str
//!
//! ```
//! type Buffer = to_str::Buffer64;
//!
//! let mut buf = String::new();
//! let _ = to_str::fmt!(Buffer, buf, 5usize);
//! assert_eq!(buf, "5");
//!
//! buf.push_str(Buffer::fmt(0usize).as_str());
//! assert_eq!(buf, "50");
//! buf.push_str(Buffer::fmt(&5usize).as_str());
//! assert_eq!(buf, "505");
//! buf.push_str(Buffer::fmt(&mut 0usize).as_str());
//! assert_eq!(buf, "5050");
//! ```

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![no_std]

mod buffer;
mod numeric;

pub use buffer::Buffer;

///Alias to buffer that can be used to write `128` bit integers
pub type Buffer128 = Buffer<[u8; i128::TEXT_SIZE]>;
///Alias to buffer that can be used to write `64` bit integers
pub type Buffer64 = Buffer<[u8; i64::TEXT_SIZE]>;

///Describes conversion to string
pub trait ToStr {
    ///Max size in bytes to hold the string
    ///
    ///Implementation MUST guarantee that this size of buffer is enough to fit any possible textual
    ///representation
    const TEXT_SIZE: usize;

    ///Writes textual representation to the buffer
    ///
    ///Returns `str` stored in the provided `buffer`
    ///
    ///Can panic, if buffer is not sufficient.
    ///Or write only partially
    ///
    ///Implementation is allowed to write any part of the buffer.
    ///It is not allowed to read it, unless it was written already.
    ///
    ///# Safety:
    ///
    ///Debug builds must never invoke UB when calling this function.
    ///
    ///UB in release mode is fine if one wants to write efficient code.
    fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str;

    #[inline]
    ///Performs textual conversion by writing to the buffer, if possible.
    ///
    ///If not possible MUST return `None`
    ///
    ///By default returns `None` if buffer size is below `TEXT_SIZE`
    ///Otherwise calls `to_str()` while passing buffer as it is
    fn to_str_if<'a>(&self, buffer: &'a mut [u8]) -> Option<&'a str> {
        if buffer.len() < Self::TEXT_SIZE {
            None
        } else {
            Some(self.to_str(buffer))
        }
    }
}

impl<'a, T: ?Sized + ToStr> ToStr for &'a T {
    const TEXT_SIZE: usize = T::TEXT_SIZE;

    #[inline(always)]
    fn to_str<'b>(&self, buffer: &'b mut [u8]) -> &'b str {
        (&**self).to_str(buffer)
    }
}

impl<'a, T: ?Sized + ToStr> ToStr for &'a mut T {
    const TEXT_SIZE: usize = T::TEXT_SIZE;

    #[inline(always)]
    fn to_str<'b>(&self, buffer: &'b mut [u8]) -> &'b str {
        (&**self).to_str(buffer)
    }
}

#[macro_export]
///Formats value via specified buffer type.
///
///## Arguments:
///
///- `Buffer` type, created via `impl_buffer` macro;
///- `Writer` value that implements `core::fmt::Write` trait;
///- `Value` to format, which implements `ToStr` trait
macro_rules! fmt {
    ($buf:ty, $w:expr, $t:expr) => {
        core::fmt::Write::write_str(&mut $w, <$buf>::fmt($t).as_str())
    }
}
