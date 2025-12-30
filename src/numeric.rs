use crate::ToStr;

use core::{num, ptr};
use core::str::from_utf8_unchecked;

//num % 100 * 2 + 1 at most will be 200, therefore DIGITS contains this much.
static DEC_DIGITS: &[u8; 200] = b"0001020304050607080910111213141516171819\
                                  2021222324252627282930313233343536373839\
                                  4041424344454647484950515253545556575859\
                                  6061626364656667686970717273747576777879\
                                  8081828384858687888990919293949596979899";
static HEX_DIGITS: [u8; 16] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f'];
const PTR_PREFIX: [u8; 2] = [b'0', b'x'];

const fn size_of_val<T>(_: &T) -> usize {
    core::mem::size_of::<T>()
}

pub(crate) const unsafe fn write_u8_to_buf(mut num: u8, buffer_ptr: *mut u8, mut cursor: isize) -> isize {
    let digits_ptr = DEC_DIGITS.as_ptr();

    if num >= 100 {
        let index = (num as isize % 100) << 1;
        num /= 100;

        cursor -= 3;
        unsafe {
            ptr::write(buffer_ptr.offset(cursor), *digits_ptr + num);
            ptr::copy_nonoverlapping(digits_ptr.offset(index), buffer_ptr.offset(cursor + 1), 2);
        }
    } else if num <= 9 {
        cursor -= 1;
        unsafe {
            ptr::write(buffer_ptr.offset(cursor), *digits_ptr + num);
        }
    } else {
        let index = num as isize * 2;

        cursor -= 2;
        unsafe {
            ptr::copy_nonoverlapping(digits_ptr.offset(index), buffer_ptr.offset(cursor), 2);
        }
    }

    cursor
}

pub(crate) const unsafe fn write_u64_to_buf(mut num: u64, buffer_ptr: *mut u8, mut cursor: isize) -> isize {
    let digits_ptr = DEC_DIGITS.as_ptr();

    while num >= 10000 {
        let rem = (num % 10000) as isize;
        num /= 10000;

        let index1 = (rem / 100) << 1;
        let index2 = (rem % 100) << 1;
        cursor -= 4;
        unsafe {
            ptr::copy_nonoverlapping(digits_ptr.offset(index1), buffer_ptr.offset(cursor), 2);
            ptr::copy_nonoverlapping(digits_ptr.offset(index2), buffer_ptr.offset(cursor + 2), 2);
        }
    }

    if num >= 100 {
        let index = (num as isize % 100) << 1;
        num /= 100;

        cursor -= 2;
        unsafe {
            ptr::copy_nonoverlapping(digits_ptr.offset(index), buffer_ptr.offset(cursor), 2);
        }
    }

    if num < 10 {
        cursor -= 1;
        unsafe {
            ptr::write(buffer_ptr.offset(cursor), *digits_ptr + num as u8);
        }
    } else {
        let index = num as isize * 2;

        cursor -= 2;
        unsafe {
            ptr::copy_nonoverlapping(digits_ptr.offset(index), buffer_ptr.offset(cursor), 2);
        }
    }

    cursor
}

//Taken from https://github.com/dtolnay/itoa for a better x128 divisions
//
//Ref: https://github.com/dtolnay/itoa/blob/3091ce69da35e9c8a8ff29702ea3310af30684e4/src/udiv128.rs#L1
#[inline]
const fn udivmod_1e19(num: &mut u128) -> u64 {
    const DIV: u64 = 10_000_000_000_000_000_000;

    const fn u128_mulhi(x: u128, y: u128) -> u128 {
        let x_lo = x as u64;
        let x_hi = (x >> 64) as u64;
        let y_lo = y as u64;
        let y_hi = (y >> 64) as u64;

        // handle possibility of overflow
        let carry = (x_lo as u128 * y_lo as u128) >> 64;
        let m = x_lo as u128 * y_hi as u128 + carry;
        let high1 = m >> 64;

        let m_lo = m as u64;
        let high2 = (x_hi as u128 * y_lo as u128 + m_lo as u128) >> 64;

        x_hi as u128 * y_hi as u128 + high1 + high2
    }

    let quot = if *num < 1 << 83 {
        ((*num >> 19) as u64 / (DIV >> 19)) as u128
    } else {
        u128_mulhi(*num, 156927543384667019095894735580191660403) >> 62
    };

    let rem = (*num - quot * DIV as u128) as u64;
    *num = quot;

    rem
}

pub(crate) const unsafe fn write_u128_to_buf(mut num: u128, buffer_ptr: *mut u8, mut cursor: isize) -> isize {
    const U64_TEXT_SIZE: isize = u64::TEXT_SIZE as isize;
    const U64_TEXT_MAX_WRITTEN: isize = u64::TEXT_SIZE as isize - 1;

    let digits_ptr = DEC_DIGITS.as_ptr();

    let mut offset = cursor - u64::TEXT_SIZE as isize;
    let mut written = unsafe {
        U64_TEXT_SIZE - write_u64_to_buf(udivmod_1e19(&mut num), buffer_ptr.offset(offset), U64_TEXT_SIZE)
    };

    cursor -= written;

    if num != 0 {
        written = (U64_TEXT_MAX_WRITTEN - written) as isize;
        cursor -= written;
        unsafe {
            ptr::write_bytes(buffer_ptr.offset(cursor), *digits_ptr, written as _);
        }

        offset = cursor - u64::TEXT_SIZE as isize;
        written = unsafe {
            U64_TEXT_SIZE - write_u64_to_buf(udivmod_1e19(&mut num), buffer_ptr.offset(offset), U64_TEXT_SIZE)
        };

        cursor -= written;

        if num != 0 {
            written = (U64_TEXT_MAX_WRITTEN - written) as isize;
            cursor -= written;
            unsafe {
                ptr::write_bytes(buffer_ptr.offset(cursor), *digits_ptr, written as _);
            }

            // There is at most one digit left
            // because u128::max / 10^19 / 10^19 is 3.
            cursor -= 1;
            unsafe {
                *buffer_ptr.offset(cursor) = (num as u8) + b'0';
            }
        }
    }

    cursor
}

const unsafe fn write_hex_to_buf(mut num: usize, buffer_ptr: *mut u8, mut cursor: isize) -> isize {
    const BASE: usize = 4;
    const BASE_DIGIT: usize = (1 << BASE) - 1;
    let digits_ptr = HEX_DIGITS.as_ptr();

    loop {
        let digit = num & BASE_DIGIT;
        cursor -= 1;
        unsafe {
            ptr::write(buffer_ptr.offset(cursor), *digits_ptr.add(digit));
        }
        num >>= BASE;

        if num == 0 {
            break;
        }
    }

    cursor
}

#[inline(always)]
pub(crate) const unsafe fn write_ptr_to_buf(num: usize, buffer_ptr: *mut u8, mut cursor: isize) -> isize {
    const PTR_PREFIX_SIZE: usize = size_of_val(&PTR_PREFIX);
    cursor = unsafe {
        write_hex_to_buf(num, buffer_ptr, cursor)
    };
    cursor -= PTR_PREFIX_SIZE as isize;

    unsafe {
        ptr::copy_nonoverlapping(PTR_PREFIX.as_ptr(), buffer_ptr.offset(cursor), PTR_PREFIX_SIZE);
    }

    cursor
}

macro_rules! impl_unsigned {
    ($t:ty: $max:expr; $conv:ident($($cv_t:tt)*)) => {
        unsafe impl ToStr for $t {
            const TEXT_SIZE: usize = $max;

            #[inline]
            fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
                debug_assert!(buffer.len() >= Self::TEXT_SIZE);

                unsafe {
                    let offset = $conv((*self) $($cv_t)*, buffer.as_mut_ptr(), buffer.len() as isize) as usize;
                    from_utf8_unchecked(&buffer[offset..])
                }
            }
        }
    }
}

impl_unsigned!(u8: 3; write_u8_to_buf(as u8));
impl_unsigned!(u16: 5; write_u64_to_buf(as u64));
impl_unsigned!(u32: 10; write_u64_to_buf(as u64));
impl_unsigned!(u64: 20; write_u64_to_buf(as u64));
impl_unsigned!(u128: 39; write_u128_to_buf(as u128));

unsafe impl ToStr for usize {
    #[cfg(target_pointer_width = "16")]
    const TEXT_SIZE: usize = <u16 as ToStr>::TEXT_SIZE;
    #[cfg(target_pointer_width = "32")]
    const TEXT_SIZE: usize = <u32 as ToStr>::TEXT_SIZE;
    #[cfg(target_pointer_width = "64")]
    const TEXT_SIZE: usize = <u64 as ToStr>::TEXT_SIZE;

    #[inline]
    fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
        debug_assert!(buffer.len() >= Self::TEXT_SIZE);

        unsafe {
            let offset = write_u64_to_buf(*self as u64, buffer.as_mut_ptr(), buffer.len() as isize) as usize;
            from_utf8_unchecked(&buffer[offset..])
        }
    }
}

macro_rules! impl_signed {
    ($t:ty as $st:ty where $conv:ident as $cv_t:ty) => {
        unsafe impl ToStr for $t {
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
impl_signed!(i128 as u128 where write_u128_to_buf as u128);

unsafe impl ToStr for isize {
    #[cfg(target_pointer_width = "16")]
    const TEXT_SIZE: usize = <i16 as ToStr>::TEXT_SIZE;
    #[cfg(target_pointer_width = "32")]
    const TEXT_SIZE: usize = <i32 as ToStr>::TEXT_SIZE;
    #[cfg(target_pointer_width = "64")]
    const TEXT_SIZE: usize = <i64 as ToStr>::TEXT_SIZE;

    #[inline]
    fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
        if self.is_negative() {
            debug_assert!(buffer.len() >= Self::TEXT_SIZE);

            #[cfg(target_pointer_width = "16")]
            let abs = 0i16.wrapping_sub(*self as i16);
            #[cfg(target_pointer_width = "32")]
            let abs = 0i32.wrapping_sub(*self as i32);
            #[cfg(target_pointer_width = "64")]
            let abs = 0i64.wrapping_sub(*self as i64);

            unsafe {
                let offset = write_u64_to_buf(abs as _, buffer.as_mut_ptr(), buffer.len() as isize) - 1;
                ptr::write(buffer.as_mut_ptr().offset(offset), b'-');
                from_utf8_unchecked(&mut buffer[offset as usize..])
            }

        } else {
            #[cfg(target_pointer_width = "16")]
            let unsigned = *self as i16;
            #[cfg(target_pointer_width = "32")]
            let unsigned = *self as i32;
            #[cfg(target_pointer_width = "64")]
            let unsigned = *self as i64;

            ToStr::to_str(&unsigned, buffer)
        }
    }
}

unsafe impl<T> ToStr for *const T {
    const TEXT_SIZE: usize = usize::TEXT_SIZE + 2;

    #[inline]
    fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
        debug_assert!(buffer.len() >= Self::TEXT_SIZE);

        unsafe {
            let offset = write_ptr_to_buf(*self as usize, buffer.as_mut_ptr(), buffer.len() as isize) as usize;
            from_utf8_unchecked(&buffer[offset..])
        }
    }
}

unsafe impl<T> ToStr for *mut T {
    const TEXT_SIZE: usize = usize::TEXT_SIZE + 2;

    #[inline(always)]
    fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
        (*self as *const T).to_str(buffer)
    }
}

unsafe impl<T> ToStr for core::sync::atomic::AtomicPtr<T> {
    const TEXT_SIZE: usize = usize::TEXT_SIZE + 2;

    #[inline(always)]
    fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
        self.load(core::sync::atomic::Ordering::Acquire).to_str(buffer)
    }
}

unsafe impl<T> ToStr for ptr::NonNull<T> {
    const TEXT_SIZE: usize = usize::TEXT_SIZE + 2;

    #[inline(always)]
    fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
        self.as_ptr().to_str(buffer)
    }
}

macro_rules! impl_non_zero_repr {
    ($($t:ty: $repr:ty);* $(;)?) => {
        $(
        unsafe impl ToStr for $t {
            const TEXT_SIZE: usize = {
                assert!(core::mem::size_of::<$t>() == core::mem::size_of::<$repr>(), "NonZero type doesn't match Repr type");
                <$repr as ToStr>::TEXT_SIZE
            };

            #[inline(always)]
            fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
                ToStr::to_str(&(*self).get(), buffer)
            }
        }
        )*
    }
}

impl_non_zero_repr!(
    num::NonZeroU8: u8;
    num::NonZeroU16: u16;
    num::NonZeroU32: u32;
    num::NonZeroU64: u64;
    num::NonZeroU128: u128;
    num::NonZeroUsize: usize;

    num::NonZeroI8: i8;
    num::NonZeroI16: i16;
    num::NonZeroI32: i32;
    num::NonZeroI64: i64;
    num::NonZeroI128: i128;
    num::NonZeroIsize: isize;
);
