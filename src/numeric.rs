use crate::ToStr;

use core::ptr;
use core::str::from_utf8_unchecked;

//num % 100 * 2 + 1 at most will be 200, therefore DIGITS contains this much.
const DIGITS: *const u8 = b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899" as *const u8;

impl ToStr for u8 {
    const TEXT_SIZE: usize = 3;

    fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
        debug_assert!(buffer.len() >= Self::TEXT_SIZE);

        let mut this = *self;
        if this <= 9 {
            unsafe {
                ptr::write(buffer.as_mut_ptr(), *DIGITS + this);
                from_utf8_unchecked(&buffer[..1])
            }
        } else if this >= 100 {
            let index = (this as isize % 100) << 1;
            this = this / 100;

            unsafe {
                ptr::write(buffer.as_mut_ptr(), *DIGITS + this);
                ptr::copy_nonoverlapping(DIGITS.offset(index), buffer.as_mut_ptr().add(1), 2);
                from_utf8_unchecked(&buffer[..3])
            }
        } else {
            let index = this as isize * 2;
            // 10..99
            unsafe {
                ptr::copy_nonoverlapping(DIGITS.offset(index), buffer.as_mut_ptr(), 2);
                from_utf8_unchecked(&buffer[..2])
            }
        }
    }
}

macro_rules! impl_unsigned {
    ($t:ty: $max:expr) => {
        impl ToStr for $t {
            const TEXT_SIZE: usize = $max;

            fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
                debug_assert!(buffer.len() >= Self::TEXT_SIZE);

                let mut this = *self;
                let mut cursor = unsafe {
                    buffer.as_mut_ptr().add(buffer.len())
                };

                while this >= 10000 {
                    let rem = (this % 10000) as isize;
                    this /= 10000;

                    let index1 = (rem / 100) << 1;
                    let index2 = (rem % 100) << 1;
                    unsafe {
                        cursor = cursor.offset(-4);
                        ptr::copy_nonoverlapping(DIGITS.offset(index1), cursor, 2);
                        ptr::copy_nonoverlapping(DIGITS.offset(index2), cursor.offset(2), 2);
                    }
                }

                let mut this = this as isize;

                if this >= 100 {
                    let index = (this % 100) << 1;
                    this /= 100;

                    unsafe {
                        cursor = cursor.offset(-2);
                        ptr::copy_nonoverlapping(DIGITS.offset(index), cursor, 2);
                    }
                }

                if this <= 9 {
                    unsafe {
                        cursor = cursor.offset(-1);
                        ptr::write(cursor, *DIGITS + this as u8);
                    }
                } else {
                    let index = this * 2;

                    unsafe {
                        cursor = cursor.offset(-2);
                        ptr::copy_nonoverlapping(DIGITS.offset(index), cursor, 2);
                    }
                }

                unsafe {
                    from_utf8_unchecked(core::slice::from_raw_parts(cursor, buffer.len() - (cursor as usize - buffer.as_mut_ptr() as usize)))
                }
            }
        }
    }
}

impl_unsigned!(u16: 5);
impl_unsigned!(u32: 10);
impl_unsigned!(u64: 20);
#[cfg(target_pointer_width = "16")]
impl_unsigned!(usize: 5);
#[cfg(target_pointer_width = "32")]
impl_unsigned!(usize: 10);
#[cfg(target_pointer_width = "64")]
impl_unsigned!(usize: 20);
impl_unsigned!(u128: 39);

macro_rules! impl_signed {
    ($t:ty as $st:ty) => {
        impl ToStr for $t {
            const TEXT_SIZE: usize = <$st>::TEXT_SIZE + 1;

            #[inline]
            fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
                if self.is_negative() {
                    debug_assert!(buffer.len() >= Self::TEXT_SIZE);

                    let abs = (0 as $st).wrapping_sub(*self as $st);
                    let cursor = buffer.len() - abs.to_str(&mut buffer[1..]).len() - 1; //-1 for sign
                    unsafe {
                        ptr::write(buffer.as_mut_ptr().add(cursor), b'-');
                        from_utf8_unchecked(&mut buffer[cursor..])
                    }

                } else {
                    (*self as $st).to_str(buffer)
                }
            }
        }
    }
}

impl_signed!(i16 as u16);
impl_signed!(i32 as u32);
impl_signed!(i64 as u64);
#[cfg(target_pointer_width = "16")]
impl_signed!(isize as u16);
#[cfg(target_pointer_width = "32")]
impl_signed!(isize as u32);
#[cfg(target_pointer_width = "64")]
impl_signed!(isize as u64);
impl_signed!(i128 as u128);

impl ToStr for i8 {
    const TEXT_SIZE: usize = u8::TEXT_SIZE + 1;

    #[inline]
    fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
        if self.is_negative() {
            debug_assert!(buffer.len() >= Self::TEXT_SIZE);

            let abs = 0u8.wrapping_sub(*self as u8);
            unsafe {
                ptr::write(buffer.as_mut_ptr(), b'-');
                let len = abs.to_str(&mut buffer[1..]).len() + 1;
                from_utf8_unchecked(&mut buffer[..len])
            }
        } else {
            (*self as u8).to_str(buffer)
        }
    }
}
