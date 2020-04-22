///Interface to static storage for text.
pub unsafe trait TextBuffer: Sized {
    ///Storage's size
    const STORAGE_SIZE: usize;

    ///Creates new instance
    fn new() -> Self;

    #[inline(always)]
    ///Returns length
    fn len(&self) -> usize {
        Self::STORAGE_SIZE
    }

    ///Returns pointer to the first element
    fn as_ptr(&self) -> *const u8;

    #[inline(always)]
    ///Returns mutable pointer to the first element
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_ptr() as *mut u8
    }
}

#[macro_export]
///Generates `Buffer` struct with provided name and capacity
///Buffer size is limited by `u8::max_value()`
macro_rules! impl_buffer {
    ($name:ident; $size:expr) => {
        ///Buffer to hold textual representation of value
        pub struct $name {
            inner: core::mem::MaybeUninit<[u8; $size]>,
            offset: u8,
        }

        $crate::static_assert!($size as usize <= u8::max_value() as usize);

        impl $name {
            ///Buffer's size
            const SIZE: usize = $size;

            #[inline]
            ///Creates new instance
            pub const fn new() -> Self {
                Self {
                    inner: core::mem::MaybeUninit::uninit(),
                    offset: $size,
                }
            }

            #[inline]
            ///Returns whether buffer holds enough space to hold specified type
            pub const fn len(&self) -> usize {
                Self::SIZE
            }

            #[inline]
            pub const fn as_ptr(&self) -> *const u8 {
                &self.inner as *const _ as *const u8
            }

            #[inline(always)]
            ///Access str from underlying storage
            ///
            ///Returns empty if nothing has been written into buffer yet.
            pub fn as_str(&self) -> &str {
                unsafe {
                    let slice = core::slice::from_raw_parts(self.as_ptr().offset(self.offset as isize), self.len() - self.offset as usize);
                    core::str::from_utf8_unchecked(slice)
                }
            }

            #[inline]
            ///Formats value into buffer.
            pub fn format<T: $crate::ToStr>(&mut self, val: &T) -> &str {
                debug_assert!(T::TEXT_SIZE <= self.len());

                val.to_str(unsafe {
                    &mut *core::ptr::slice_from_raw_parts_mut(self.as_ptr() as *mut u8, self.len())
                })
            }
        }

        unsafe impl $crate::TextBuffer for $name {
            const STORAGE_SIZE: usize = $size;

            #[inline(always)]
            fn new() -> Self {
                $name::new()
            }

            #[inline(always)]
            fn len(&self) -> usize {
                Self::STORAGE_SIZE
            }

            #[inline(always)]
            fn as_ptr(&self) -> *const u8 {
                $name::as_ptr(self)
            }
        }

        impl AsRef<str> for $name {
            #[inline(always)]
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl core::fmt::Display for $name {
            #[inline(always)]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl core::fmt::Debug for $name {
            #[inline(always)]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    }
}
