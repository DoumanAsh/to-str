use crate::ToStr;

use core::{borrow, ptr};
use core::str::from_utf8_unchecked;

//num % 100 * 2 + 1 at most will be 200, therefore DIGITS contains this much.
static DEC_DIGITS: &[u8; 200] = b"0001020304050607080910111213141516171819\
                                  2021222324252627282930313233343536373839\
                                  4041424344454647484950515253545556575859\
                                  6061626364656667686970717273747576777879\
                                  8081828384858687888990919293949596979899";

///Number to string conversion
pub trait NumToStr: Sized {
    ///Specifies storage for static buffer
    type Storage: borrow::BorrowMut<[u8]> + Copy;
    ///Writes number to the buffer
    fn to_str_buffer(self, buffer: &mut crate::Buffer<Self::Storage>);
    ///Converts number to string, returning static buffer with contents
    #[inline]
    fn to_str(self) -> crate::Buffer<Self::Storage> {
        let mut result = crate::Buffer::<Self::Storage>::new_uninit();
        Self::to_str_buffer(self, &mut result);
        result
    }
}

unsafe fn write_u8_to_buf(mut num: u8, buffer_ptr: *mut u8, mut cursor: isize) -> isize {
    let digits_ptr = DEC_DIGITS.as_ptr();

    if num >= 100 {
        let index = (num as isize % 100) << 1;
        num /= 100;

        cursor -= 3;
        ptr::write(buffer_ptr.offset(cursor), *digits_ptr + num);
        ptr::copy_nonoverlapping(digits_ptr.offset(index), buffer_ptr.offset(cursor + 1), 2);
    } else if num <= 9 {
        cursor -= 1;
        ptr::write(buffer_ptr.offset(cursor), *digits_ptr + num);
    } else {
        let index = num as isize * 2;

        cursor -= 2;
        ptr::copy_nonoverlapping(digits_ptr.offset(index), buffer_ptr.offset(cursor), 2);
    }

    cursor
}

unsafe fn write_u64_to_buf(mut num: u64, buffer_ptr: *mut u8, mut cursor: isize) -> isize {
    let digits_ptr = DEC_DIGITS.as_ptr();

    while num >= 10000 {
        let rem = (num % 10000) as isize;
        num /= 10000;

        let index1 = (rem / 100) << 1;
        let index2 = (rem % 100) << 1;
        cursor -= 4;
        ptr::copy_nonoverlapping(digits_ptr.offset(index1), buffer_ptr.offset(cursor), 2);
        ptr::copy_nonoverlapping(digits_ptr.offset(index2), buffer_ptr.offset(cursor + 2), 2);
    }

    let mut num = num as isize;

    if num >= 100 {
        let index = (num % 100) << 1;
        num /= 100;

        cursor -= 2;
        ptr::copy_nonoverlapping(digits_ptr.offset(index), buffer_ptr.offset(cursor), 2);
    }

    if num <= 9 {
        cursor -= 1;
        ptr::write(buffer_ptr.offset(cursor), *digits_ptr + num as u8);
    } else {
        let index = num * 2;

        cursor -= 2;
        ptr::copy_nonoverlapping(digits_ptr.offset(index), buffer_ptr.offset(cursor), 2);
    }

    cursor
}

#[inline]
///Taken from https://github.com/dtolnay/itoa for a better x128 divisions
pub fn udivmod_1e19(num: &mut u128) -> u64 {
    const DIV: u64 = 10_000_000_000_000_000_000;

    let high = (*num >> 64) as u64;
    if high == 0 {
        let low = *num as u64;
        *num = (low / DIV) as u128;
        return low % DIV;
    }

    let sr = 65 - high.leading_zeros();

    // 2 <= sr <= 65
    let mut q: u128 = *num << (128 - sr);
    let mut r: u128 = *num >> sr;
    let mut carry: u64 = 0;

    // Don't use a range because they may generate references to memcpy in unoptimized code
    //
    // Loop invariants:  r < d; carry is 0 or 1
    let mut i = 0;
    while i < sr {
        i += 1;

        // r:q = ((r:q) << 1) | carry
        r = (r << 1) | (q >> 127);
        q = (q << 1) | carry as u128;

        // carry = 0
        // if r >= d {
        //     r -= d;
        //     carry = 1;
        // }
        let s = (DIV as u128).wrapping_sub(r).wrapping_sub(1) as i128 >> 127;
        carry = (s & 1) as u64;
        r -= (DIV as u128) & s as u128;
    }

    *num = (q << 1) | carry as u128;
    r as u64
}

unsafe fn write_u128_to_buf(mut num: u128, buffer_ptr: *mut u8, mut cursor: isize) -> isize {
    const U64_TEXT_SIZE: isize = u64::TEXT_SIZE as isize;
    const U64_TEXT_MAX_WRITTEN: isize = u64::TEXT_SIZE as isize - 1;
    let digits_ptr = DEC_DIGITS.as_ptr();

    let mut offset = cursor - u64::TEXT_SIZE as isize;
    let mut written = U64_TEXT_SIZE - write_u64_to_buf(udivmod_1e19(&mut num), buffer_ptr.offset(offset), U64_TEXT_SIZE);

    cursor -= written;

    if num != 0 {
        ptr::write_bytes(buffer_ptr.offset(cursor), *digits_ptr, (U64_TEXT_MAX_WRITTEN - written) as usize);

        offset = cursor - u64::TEXT_SIZE as isize;
        written = U64_TEXT_SIZE - write_u64_to_buf(udivmod_1e19(&mut num), buffer_ptr.offset(offset), U64_TEXT_SIZE);

        cursor -= written;

        if num != 0 {
            ptr::write_bytes(buffer_ptr.offset(cursor), *digits_ptr, (U64_TEXT_MAX_WRITTEN - written) as usize);

            // There is at most one digit left
            // because u128::max / 10^19 / 10^19 is 3.
            cursor -= 1;
            *buffer_ptr.offset(cursor) = (num as u8) + b'0';
        }
    }

    cursor
}

macro_rules! impl_unsigned {
    ($t:ty: $max:expr; $conv:ident as $cv_t:ident) => {
        impl NumToStr for $t {
            type Storage = [u8; $max];

            #[inline]
            fn to_str_buffer(self, buffer: &mut crate::Buffer<Self::Storage>) {
                let offset = unsafe {
                    $conv(self as $cv_t, buffer.as_mut_ptr(), buffer.inner.len() as isize)
                };

                buffer.offset = offset as u8;
            }
        }

        impl ToStr for $t {
            const TEXT_SIZE: usize = $max;

            #[inline]
            fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
                debug_assert!(buffer.len() >= Self::TEXT_SIZE);

                unsafe {
                    let offset = $conv(*self as $cv_t, buffer.as_mut_ptr(), buffer.len() as isize) as usize;
                    from_utf8_unchecked(&buffer[offset..])
                }
            }
        }
    }
}

impl ToStr for u128 {
    const TEXT_SIZE: usize = 39;

    #[inline]
    fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
        debug_assert!(buffer.len() >= Self::TEXT_SIZE);

        unsafe {
            let offset = write_u128_to_buf(*self, buffer.as_mut_ptr(), buffer.len() as isize) as usize;
            from_utf8_unchecked(&buffer[offset..])
        }
    }
}

impl_unsigned!(u8: 3; write_u8_to_buf as u8);
impl_unsigned!(u16: 5; write_u64_to_buf as u64);
impl_unsigned!(u32: 10; write_u64_to_buf as u64);
impl_unsigned!(u64: 20; write_u64_to_buf as u64);
#[cfg(target_pointer_width = "16")]
impl_unsigned!(usize: 5; write_u64_to_buf as u64);
#[cfg(target_pointer_width = "32")]
impl_unsigned!(usize: 10; write_u64_to_buf as u64);
#[cfg(target_pointer_width = "64")]
impl_unsigned!(usize: 20; write_u64_to_buf as u64);
//Cannot use [u8; 39] due to missing traits.
//Thanks rust without generic integers
//impl_unsigned!(u128: 39; write_u128_to_buf as u128);

macro_rules! impl_signed {
    ($t:ty as $st:ty where $conv:ident as $cv_t:ty) => {
        impl NumToStr for $t {
            type Storage = [u8; <$t>::TEXT_SIZE + 1];

            #[inline]
            fn to_str_buffer(self, buffer: &mut crate::Buffer<Self::Storage>) {
                let offset = if self.is_negative() {
                    debug_assert!(buffer.len() >= Self::TEXT_SIZE);

                    let abs = (0 as $st).wrapping_sub(self as $st);
                    unsafe {
                        let offset = $conv(abs as $cv_t, buffer.as_mut_ptr(), buffer.len() as isize) - 1;
                        ptr::write(buffer.as_mut_ptr().offset(offset), b'-');
                        offset
                    }
                } else {
                    unsafe {
                        $conv(self as $cv_t, buffer.as_mut_ptr(), buffer.inner.len() as isize)
                    }
                };

                buffer.offset = offset as u8;
            }
        }

        impl ToStr for $t {
            const TEXT_SIZE: usize = <$st>::TEXT_SIZE + 1;

            #[inline]
            fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
                if self.is_negative() {
                    debug_assert!(buffer.len() >= Self::TEXT_SIZE);

                    let abs = (0 as $st).wrapping_sub(*self as $st);
                    unsafe {
                        let offset = $conv(abs as $cv_t, buffer.as_mut_ptr(), buffer.len() as isize) - 1;
                        ptr::write(buffer.as_mut_ptr().offset(offset), b'-');
                        from_utf8_unchecked(&mut buffer[offset as usize..])
                    }

                } else {
                    ToStr::to_str(&(*self as $st), buffer)
                }
            }
        }
    }
}

impl_signed!(i8 as u8 where write_u8_to_buf as u8);
impl_signed!(i16 as u16 where write_u64_to_buf as u64);
impl_signed!(i32 as u32 where write_u64_to_buf as u64);
impl_signed!(i64 as u64 where write_u64_to_buf as u64);
#[cfg(target_pointer_width = "16")]
impl_signed!(isize as u16 where write_u64_to_buf as u64);
#[cfg(target_pointer_width = "32")]
impl_signed!(isize as u32 where write_u64_to_buf as u64);
#[cfg(target_pointer_width = "64")]
impl_signed!(isize as u64 where write_u64_to_buf as u64);
//impl_signed!(i128 as u128);

impl ToStr for i128 {
    const TEXT_SIZE: usize = u128::TEXT_SIZE + 1;

    #[inline]
    fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
        if self.is_negative() {
            debug_assert!(buffer.len() >= Self::TEXT_SIZE);

            let abs = (0 as u128).wrapping_sub(*self as u128);
            unsafe {
                let offset = write_u128_to_buf(abs, buffer.as_mut_ptr(), buffer.len() as isize) - 1;
                ptr::write(buffer.as_mut_ptr().offset(offset), b'-');
                from_utf8_unchecked(&mut buffer[offset as usize..])
            }

        } else {
            ToStr::to_str(&(*self as u128), buffer)
        }
    }
}
