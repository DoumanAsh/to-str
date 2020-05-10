use core::mem;

///Static buffer to hold written text.
///
///Implementation of `ToStr` must write it from the end.
pub struct Buffer<T> {
    inner: core::mem::MaybeUninit<T>,
    offset: u8,
}

impl<S: Sized> Buffer<S> {
    #[inline]
    ///Creates new instance
    pub const fn new() -> Self {
        Self {
            inner: core::mem::MaybeUninit::uninit(),
            offset: 0,
        }
    }

    #[inline]
    ///Returns pointer  to the beginning of underlying buffer
    pub const fn as_ptr(&self) -> *const u8 {
        &self.inner as *const _ as *const u8
    }

    #[inline]
    ///Returns buffer overall capacity.
    pub const fn capacity() -> usize {
        mem::size_of::<S>()
    }

    #[inline(always)]
    ///Access str from underlying storage
    ///
    ///Returns empty if nothing has been written into buffer yet.
    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = core::slice::from_raw_parts(self.as_ptr().offset(self.offset as isize), Self::capacity() - self.offset as usize);
            core::str::from_utf8_unchecked(slice)
        }
    }

    #[inline]
    ///Formats value into buffer, returning text.
    ///
    ///Buffer remembers the write, therefore `as_str()` will return the same text as last
    ///`write`
    pub fn write<T: crate::ToStr>(&mut self, val: T) -> &str {
        self.offset = (Self::capacity() - self.format(val).len()) as u8;
        self.as_str()
    }

    #[inline(always)]
    ///Formats value into buffer, returning text.
    ///
    ///Buffer remains unaware of modifications
    pub fn format<T: crate::ToStr>(&mut self, val: T) -> &str {
        //Yes, because we cannot assert statically in generics, we must go through these hacks
        //We can add this assertion once panic will be allowed inside const fn
        debug_assert!(Self::capacity() <= u8::max_value() as usize);
        debug_assert!(T::TEXT_SIZE <= Self::capacity());

        val.to_str(unsafe {
            &mut *core::ptr::slice_from_raw_parts_mut(self.as_ptr() as *mut u8, Self::capacity())
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

impl<S: Sized> AsRef<str> for Buffer<S> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<S: Sized> core::fmt::Display for Buffer<S> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<S: Sized> core::fmt::Debug for Buffer<S> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
