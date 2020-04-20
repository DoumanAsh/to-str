//! `no_std` conversion to str

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![no_std]

mod numeric;

use core::{str, slice, borrow, mem};

pub use numeric::NumToStr;

///Storage for static string
pub struct Buffer<T> {
    inner: mem::ManuallyDrop<T>,
    offset: u8,
}

impl<T: borrow::BorrowMut<[u8]>> Buffer<T> {
    ///Writes number within the buffer.
    pub fn format<N: NumToStr<Storage=T>>(&mut self, num: N) -> &str {
        num.to_str_buffer(self);
        self.as_str()
    }
}

impl<T: borrow::Borrow<[u8]>> Buffer<T> {
    ///Creates new uninitialized buffer.
    pub fn new_uninit() -> Self {
        Self {
            inner: unsafe {
                mem::ManuallyDrop::new(mem::MaybeUninit::uninit().assume_init())
            },
            offset: 0
        }
    }

    #[inline(always)]
    ///Returns buffer's len
    pub fn len(&self) -> usize {
        self.inner.borrow().len()
    }

    #[inline(always)]
    ///Returns pointer to the first element in buffer
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        &mut self.inner as *mut _ as *mut u8
    }

    #[inline(always)]
    ///Access str from underlying storage
    pub fn as_str(&self) -> &str {
        unsafe {
            let ptr = &self.inner as *const _ as *const u8;
            str::from_utf8_unchecked(slice::from_raw_parts(ptr.offset(self.offset as isize), self.len() - self.offset as usize))
        }
    }
}

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
