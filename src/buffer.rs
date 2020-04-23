#[macro_export]
///Generates `Buffer` struct with provided name and capacity.
///Buffer's capacity is limited by `u8::max_value()`
macro_rules! impl_buffer {
    ($name:ident; $size:expr) => {
        ///Buffer to hold textual representation of value
        pub struct $name {
            inner: core::mem::MaybeUninit<[u8; $size as usize]>,
            offset: u8,
        }

        impl $name {
            ///Buffer's size
            const SIZE: usize = $size as usize;

            #[inline]
            ///Creates new instance
            pub const fn new() -> Self {
                $crate::static_assert!($size as usize <= u8::max_value() as usize);

                Self {
                    inner: core::mem::MaybeUninit::uninit(),
                    offset: $size as u8,
                }
            }

            #[inline]
            ///Returns whether buffer holds enough space to hold specified type
            pub const fn len(&self) -> usize {
                Self::SIZE
            }

            #[inline]
            ///Returns pointer  to the beginning of underlying buffer
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
            ///Formats value into buffer, returning text.
            ///
            ///Buffer remembers the write, therefore `as_str()` will return the same text as last
            ///`write`
            pub fn write<T: $crate::ToStr>(&mut self, val: T) -> &str {
                self.offset = (self.len() - self.format(val).len()) as u8;
                self.as_str()
            }

            #[inline(always)]
            ///Formats value into buffer, returning text.
            ///
            ///Buffer remains unaware of modifications
            pub fn format<T: $crate::ToStr>(&mut self, val: T) -> &str {
                debug_assert!(T::TEXT_SIZE <= self.len());

                val.to_str(unsafe {
                    &mut *core::ptr::slice_from_raw_parts_mut(self.as_ptr() as *mut u8, self.len())
                })
            }

            #[inline(always)]
            ///Creates new instance with formatted value.
            pub fn fmt<T: $crate::ToStr>(val: T) -> Self {
                let mut this = Self::new();
                this.write(val);
                this
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
