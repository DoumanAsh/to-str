//! `no_std` friendly interface for conversion to str
//!
//! ```
//! type Buffer = to_str::Buffer64;
//!
//! let mut buf = String::new();
//! let _ = buf.push_str(Buffer::fmt(5usize).as_str());
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
#![allow(clippy::style)]
#![no_std]

mod buffer;
mod numeric;

pub use buffer::Buffer;

///Alias to buffer that can be used to write `128` bit integers
pub type Buffer128 = Buffer<{i128::TEXT_SIZE}>;
///Alias to buffer that can be used to write `64` bit integers
pub type Buffer64 = Buffer<{i64::TEXT_SIZE}>;
///Alias to buffer that can be used to write `32` bit integers
pub type Buffer32 = Buffer<{i32::TEXT_SIZE}>;
///Alias to buffer that can be used to write `isize` bit integers
pub type BufferSized = Buffer<{isize::TEXT_SIZE}>;

///Describes conversion to string
///
///This trait is unsafe due to following requirements:
///
///- Implementation must never read buffer, unless it was already written by it;
///- It writes from the end of buffer (necessary only when you use `Buffer`).
pub unsafe trait ToStr {
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

unsafe impl<T: ?Sized + ToStr> ToStr for &T {
    const TEXT_SIZE: usize = T::TEXT_SIZE;

    #[inline(always)]
    fn to_str<'b>(&self, buffer: &'b mut [u8]) -> &'b str {
        (&**self).to_str(buffer)
    }
}

unsafe impl<T: ?Sized + ToStr> ToStr for &mut T {
    const TEXT_SIZE: usize = T::TEXT_SIZE;

    #[inline(always)]
    fn to_str<'b>(&self, buffer: &'b mut [u8]) -> &'b str {
        (&**self).to_str(buffer)
    }
}
