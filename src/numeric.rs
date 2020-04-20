use crate::ToStr;

use core::ptr;
use core::str::from_utf8_unchecked;

//num % 100 * 2 + 1 at most will be 200, therefore DIGITS contains this much.
const DIGITS: &[u8] = b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899";

impl ToStr for u8 {
    const TEXT_SIZE: usize = 3;

    fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
        debug_assert!(buffer.len() >= Self::TEXT_SIZE);

        let mut this = *self;
        if this <= 9 {
            unsafe {
                ptr::write(buffer.as_mut_ptr(), DIGITS.get_unchecked(0) + this);
                from_utf8_unchecked(&buffer[..1])
            }
        } else if this >= 100 {
            let index = (this % 100 * 2) as usize;
            this = this / 100;

            unsafe {
                ptr::write(buffer.as_mut_ptr(), DIGITS.get_unchecked(0) + this);
                ptr::write(buffer.as_mut_ptr().add(1), *(DIGITS.get_unchecked(index)));
                ptr::write(buffer.as_mut_ptr().add(2), *(DIGITS.get_unchecked(index + 1)));
                from_utf8_unchecked(&buffer[..3])
            }
        } else {
            let index = this as usize * 2;
            // 10..99
            unsafe {
                ptr::write(buffer.as_mut_ptr(), *(DIGITS.get_unchecked(index)));
                ptr::write(buffer.as_mut_ptr().add(1), *(DIGITS.get_unchecked(index + 1)));
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
                let mut cursor = buffer.len() - 1;

                while this >= 100 {
                    let index = (this % 100 * 2) as usize;
                    this /= 100;

                    unsafe {
                        ptr::write(buffer.as_mut_ptr().add(cursor), *(DIGITS.get_unchecked(index + 1)));
                        cursor -= 1;
                        ptr::write(buffer.as_mut_ptr().add(cursor), *(DIGITS.get_unchecked(index)));
                    }

                    cursor -= 1;
                }

                if this <= 9 {
                    unsafe {
                        ptr::write(buffer.as_mut_ptr().add(cursor), DIGITS.get_unchecked(0) + this as u8);
                    }
                } else {
                    let index = this as usize * 2;

                    unsafe {
                        ptr::write(buffer.as_mut_ptr().add(cursor), *(DIGITS.get_unchecked(index + 1)));
                        cursor -= 1;
                        ptr::write(buffer.as_mut_ptr().add(cursor), *(DIGITS.get_unchecked(index)));
                    }

                }

                unsafe {
                    from_utf8_unchecked(&buffer[cursor..])
                }
            }
        }
    }
}

impl_unsigned!(u16: 5);
impl_unsigned!(u32: 10);
impl_unsigned!(u64: 20);
impl_unsigned!(u128: 39);

macro_rules! impl_signed {
    ($t:ty as $st:ty) => {
        impl ToStr for $t {
            const TEXT_SIZE: usize = <$st>::TEXT_SIZE + 1;

            #[inline]
            fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
                if self.is_negative() {
                    debug_assert!(buffer.len() >= Self::TEXT_SIZE);

                    let mut abs = *self as $st;
                    abs = (0 as $st).wrapping_sub(abs);
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
impl_signed!(i128 as u128);

impl ToStr for i8 {
    const TEXT_SIZE: usize = u8::TEXT_SIZE + 1;

    #[inline]
    fn to_str<'a>(&self, buffer: &'a mut [u8]) -> &'a str {
        if self.is_negative() {
            debug_assert!(buffer.len() >= Self::TEXT_SIZE);

            let mut abs = *self as u8;
            abs = 0u8.wrapping_sub(abs);
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

