use crate::{numeric, ToStr};
use crate::utils::AssertSizeIsLessOrEqualThan;

use core::{fmt, mem};

///Static buffer to hold written text.
///
///Implementation of `ToStr` must write it from the end.
///
///## Buffer size limit
///
///Buffer size cannot be greater than 255.
///
///Following code will fail at compile time:
///
///### Invalid buffer creation
///
///```rust,compile_fail
///to_str::Buffer::<256>::new();;
///```
///
///### Formatting with buffer overflow
///
///```rust,compile_fail
///to_str::Buffer::<1>::new().format_u8(255);
///```
pub struct Buffer<const N: usize> {
    inner: [core::mem::MaybeUninit<u8>; N],
    offset: u8,
}

impl<const N: usize> Buffer<N> {
    const VALIDATION: () = {
        assert!(Self::capacity() <= u8::MAX as usize, "Buffer capacity cannot be greater than 255");
    };

    #[inline]
    ///Creates new instance
    pub const fn new() -> Self {
        let _ = Self::VALIDATION;
        Self {
            #[cfg(debug_assertions)]
            inner: [mem::MaybeUninit::zeroed(); N],
            #[cfg(not(debug_assertions))]
            inner: [mem::MaybeUninit::uninit(); N],
            offset: 0,
        }
    }

    #[inline]
    ///Returns pointer  to the beginning of underlying buffer
    pub const fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr() as _
    }

    #[inline]
    ///Returns pointer  to the beginning of underlying buffer
    const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr() as _
    }

    #[inline]
    ///Returns buffer overall capacity.
    pub const fn capacity() -> usize {
        N
    }

    #[inline(always)]
    const fn as_offset_str(&self, offset: isize) -> &str {
        unsafe {
            let slice = core::slice::from_raw_parts(self.as_ptr().offset(offset), Self::capacity() - offset as usize);
            core::str::from_utf8_unchecked(slice)
        }
    }

    #[inline(always)]
    ///Access str from underlying storage
    ///
    ///Returns empty if nothing has been written into buffer yet.
    pub const fn as_str(&self) -> &str {
        self.as_offset_str(self.offset as _)
    }

    #[inline]
    ///Formats value into buffer, returning text.
    ///
    ///Buffer remembers the write, therefore `as_str()` will return the same text as last
    ///`write`
    pub fn write<T: ToStr>(&mut self, val: T) -> &str {
        self.offset = (Self::capacity() - self.format(val).len()) as u8;
        self.as_str()
    }

    #[inline(always)]
    ///Formats value into buffer, returning text.
    ///
    ///Buffer remains unaware of modifications
    pub fn format<T: ToStr>(&mut self, val: T) -> &str {
        let _ = Self::VALIDATION;
        assert!(T::TEXT_SIZE <= Self::capacity());

        val.to_str(unsafe {
            &mut *core::ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), Self::capacity())
        })
    }
    #[inline(always)]
    ///Creates new instance with formatted value.
    pub fn fmt<T: crate::ToStr>(val: T) -> Self {
        let mut this = Self::new();
        this.write(val);
        this
    }
}

impl<const N: usize> Buffer<N> {
    #[inline(always)]
    ///Specialized const format of `u8` value into buffer, returning text.
    pub const fn format_u8(&mut self, val: u8) -> &str {
        let _ = AssertSizeIsLessOrEqualThan::<{<u8 as ToStr>::TEXT_SIZE}, N>::RESULT;

        numeric::unsigned::u8(val, &mut self.inner)
    }

    #[inline(always)]
    ///Creates new instance with formatted value.
    pub const fn fmt_u8(val: u8) -> Self {
        let _ = AssertSizeIsLessOrEqualThan::<{<u8 as ToStr>::TEXT_SIZE}, N>::RESULT;

        let mut this = Self::new();
        this.offset = (Self::capacity() - numeric::unsigned::u8(val, &mut this.inner).len()) as u8;
        this
    }

    #[inline(always)]
    ///Specialized const format of `u16` value into buffer, returning text.
    pub const fn format_u16(&mut self, val: u16) -> &str {
        let _ = AssertSizeIsLessOrEqualThan::<{<u16 as ToStr>::TEXT_SIZE}, N>::RESULT;

        numeric::unsigned::u16(val, &mut self.inner)
    }

    #[inline(always)]
    ///Creates new instance with formatted value.
    pub const fn fmt_u16(val: u16) -> Self {
        let _ = AssertSizeIsLessOrEqualThan::<{<u16 as ToStr>::TEXT_SIZE}, N>::RESULT;

        let mut this = Self::new();
        this.offset = (Self::capacity() - numeric::unsigned::u16(val, &mut this.inner).len()) as u8;
        this
    }

    #[inline(always)]
    ///Specialized const format of `u32` value into buffer, returning text.
    pub const fn format_u32(&mut self, val: u32) -> &str {
        let _ = AssertSizeIsLessOrEqualThan::<{<u32 as ToStr>::TEXT_SIZE}, N>::RESULT;

        numeric::unsigned::u32(val, &mut self.inner)
    }

    #[inline(always)]
    ///Creates new instance with formatted value.
    pub const fn fmt_u32(val: u32) -> Self {
        let _ = AssertSizeIsLessOrEqualThan::<{<u32 as ToStr>::TEXT_SIZE}, N>::RESULT;

        let mut this = Self::new();
        this.offset = (Self::capacity() - numeric::unsigned::u32(val, &mut this.inner).len()) as u8;
        this
    }

    #[inline(always)]
    ///Specialized const format of `u64` value into buffer, returning text.
    pub const fn format_u64(&mut self, val: u64) -> &str {
        let _ = AssertSizeIsLessOrEqualThan::<{<u64 as ToStr>::TEXT_SIZE}, N>::RESULT;

        numeric::unsigned::u64(val, &mut self.inner)
    }

    #[inline(always)]
    ///Creates new instance with formatted value.
    pub const fn fmt_u64(val: u64) -> Self {
        let _ = AssertSizeIsLessOrEqualThan::<{<u64 as ToStr>::TEXT_SIZE}, N>::RESULT;

        let mut this = Self::new();
        this.offset = (Self::capacity() - numeric::unsigned::u64(val, &mut this.inner).len()) as u8;
        this
    }

    #[inline(always)]
    ///Specialized const format of `usize` value into buffer, returning text.
    pub const fn format_usize(&mut self, val: usize) -> &str {
        let _ = AssertSizeIsLessOrEqualThan::<{<usize as ToStr>::TEXT_SIZE}, N>::RESULT;

        numeric::unsigned::usize(val, &mut self.inner)
    }

    #[inline(always)]
    ///Creates new instance with formatted value.
    pub const fn fmt_usize(val: usize) -> Self {
        let _ = AssertSizeIsLessOrEqualThan::<{<usize as ToStr>::TEXT_SIZE}, N>::RESULT;

        let mut this = Self::new();
        this.offset = (Self::capacity() - numeric::unsigned::usize(val, &mut this.inner).len()) as u8;
        this
    }

    #[inline(always)]
    ///Specialized const format of `u128` value into buffer, returning text.
    pub const fn format_u128(&mut self, val: u128) -> &str {
        let _ = AssertSizeIsLessOrEqualThan::<{<u128 as ToStr>::TEXT_SIZE}, N>::RESULT;

        numeric::unsigned::u128(val, &mut self.inner)
    }

    #[inline(always)]
    ///Creates new instance with formatted value.
    pub const fn fmt_u128(val: u128) -> Self {
        let _ = AssertSizeIsLessOrEqualThan::<{<u128 as ToStr>::TEXT_SIZE}, N>::RESULT;

        let mut this = Self::new();
        this.offset = (Self::capacity() - numeric::unsigned::u128(val, &mut this.inner).len()) as u8;
        this
    }
}

impl<const N: usize> Buffer<N> {
    #[inline(always)]
    ///Specialized const format of `i8` value into buffer, returning text.
    pub const fn format_i8(&mut self, val: i8) -> &str {
        let _ = AssertSizeIsLessOrEqualThan::<{<i8 as ToStr>::TEXT_SIZE}, N>::RESULT;

        numeric::signed::i8(val, &mut self.inner)
    }

    #[inline(always)]
    ///Creates new instance with formatted value.
    pub const fn fmt_i8(val: i8) -> Self {
        let _ = AssertSizeIsLessOrEqualThan::<{<i8 as ToStr>::TEXT_SIZE}, N>::RESULT;

        let mut this = Self::new();
        this.offset = (Self::capacity() - numeric::signed::i8(val, &mut this.inner).len()) as u8;
        this
    }

    #[inline(always)]
    ///Specialized const format of `i16` value into buffer, returning text.
    pub const fn format_i16(&mut self, val: i16) -> &str {
        let _ = AssertSizeIsLessOrEqualThan::<{<i16 as ToStr>::TEXT_SIZE}, N>::RESULT;

        numeric::signed::i16(val, &mut self.inner)
    }

    #[inline(always)]
    ///Creates new instance with formatted value.
    pub const fn fmt_i16(val: i16) -> Self {
        let _ = AssertSizeIsLessOrEqualThan::<{<i16 as ToStr>::TEXT_SIZE}, N>::RESULT;

        let mut this = Self::new();
        this.offset = (Self::capacity() - numeric::signed::i16(val, &mut this.inner).len()) as u8;
        this
    }

    #[inline(always)]
    ///Specialized const format of `i32` value into buffer, returning text.
    pub const fn format_i32(&mut self, val: i32) -> &str {
        let _ = AssertSizeIsLessOrEqualThan::<{<i32 as ToStr>::TEXT_SIZE}, N>::RESULT;

        numeric::signed::i32(val, &mut self.inner)
    }

    #[inline(always)]
    ///Creates new instance with formatted value.
    pub const fn fmt_i32(val: i32) -> Self {
        let _ = AssertSizeIsLessOrEqualThan::<{<i32 as ToStr>::TEXT_SIZE}, N>::RESULT;

        let mut this = Self::new();
        this.offset = (Self::capacity() - numeric::signed::i32(val, &mut this.inner).len()) as u8;
        this
    }

    #[inline(always)]
    ///Specialized const format of `i64` value into buffer, returning text.
    pub const fn format_i64(&mut self, val: i64) -> &str {
        let _ = AssertSizeIsLessOrEqualThan::<{<i64 as ToStr>::TEXT_SIZE}, N>::RESULT;

        numeric::signed::i64(val, &mut self.inner)
    }

    #[inline(always)]
    ///Creates new instance with formatted value.
    pub const fn fmt_i64(val: i64) -> Self {
        let _ = AssertSizeIsLessOrEqualThan::<{<i64 as ToStr>::TEXT_SIZE}, N>::RESULT;

        let mut this = Self::new();
        this.offset = (Self::capacity() - numeric::signed::i64(val, &mut this.inner).len()) as u8;
        this
    }

    #[inline(always)]
    ///Specialized const format of `isize` value into buffer, returning text.
    pub const fn format_isize(&mut self, val: isize) -> &str {
        let _ = AssertSizeIsLessOrEqualThan::<{<isize as ToStr>::TEXT_SIZE}, N>::RESULT;

        numeric::signed::isize(val, &mut self.inner)
    }

    #[inline(always)]
    ///Creates new instance with formatted value.
    pub const fn fmt_isize(val: isize) -> Self {
        let _ = AssertSizeIsLessOrEqualThan::<{<isize as ToStr>::TEXT_SIZE}, N>::RESULT;

        let mut this = Self::new();
        this.offset = (Self::capacity() - numeric::signed::isize(val, &mut this.inner).len()) as u8;
        this
    }

    #[inline(always)]
    ///Specialized const format of `i128` value into buffer, returning text.
    pub const fn format_i128(&mut self, val: i128) -> &str {
        let _ = AssertSizeIsLessOrEqualThan::<{<i128 as ToStr>::TEXT_SIZE}, N>::RESULT;

        numeric::signed::i128(val, &mut self.inner)
    }

    #[inline(always)]
    ///Creates new instance with formatted value.
    pub const fn fmt_i128(val: i128) -> Self {
        let _ = AssertSizeIsLessOrEqualThan::<{<i128 as ToStr>::TEXT_SIZE}, N>::RESULT;

        let mut this = Self::new();
        this.offset = (Self::capacity() - numeric::signed::i128(val, &mut this.inner).len()) as u8;
        this
    }
}

impl<const N: usize> AsRef<str> for Buffer<N> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> fmt::Display for Buffer<N> {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

impl<const N: usize> fmt::Debug for Buffer<N> {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), fmt)
    }
}
