//! `no_std` conversion to str

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![no_std]

mod numeric;

use core::{borrow, fmt, mem};

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
