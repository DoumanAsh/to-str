use to_str::{ToStr, Buffer128, Buffer64, Buffer8};

use core::num;
use core::fmt::Write;

const _: Buffer8 = Buffer8::fmt_u8(u8::MAX);
const _: Buffer64 = Buffer64::fmt_u64(u64::MAX);
const _: Buffer128 = Buffer128::fmt_u128(u128::MAX);

#[test]
fn should_check_const_foramt() {
    let mut buffer = Buffer128::new();

    let result = buffer.format_u8(u8::max_value());
    assert_eq!(result, "255");
    assert_eq!(result, Buffer128::fmt_u8(u8::max_value()).as_str());

    let result = buffer.format_u16(u16::max_value());
    assert_eq!(result, u16::max_value().to_string());
    assert_eq!(result, Buffer128::fmt_u16(u16::max_value()).as_str());

    let result = buffer.format_u32(u32::max_value());
    assert_eq!(result, u32::max_value().to_string());
    assert_eq!(result, Buffer128::fmt_u32(u32::max_value()).as_str());

    let result = buffer.format_u64(u64::max_value());
    assert_eq!(result, u64::max_value().to_string());
    assert_eq!(result, Buffer128::fmt_u64(u64::max_value()).as_str());

    let result = buffer.format_usize(usize::max_value());
    assert_eq!(result, usize::max_value().to_string());
    assert_eq!(result, Buffer128::fmt_usize(usize::max_value()).as_str());

    let result = buffer.format_u128(u128::max_value());
    assert_eq!(result, u128::max_value().to_string());
    assert_eq!(result, Buffer128::fmt_u128(u128::max_value()).as_str());

    let result = buffer.format_i8(i8::max_value());
    assert_eq!(result, "127");
    assert_eq!(result, Buffer128::fmt_i8(i8::max_value()).as_str());

    let result = buffer.format_i8(i8::min_value());
    assert_eq!(result, "-128");
    assert_eq!(result, Buffer128::fmt_i8(i8::min_value()).as_str());

    let result = buffer.format_i16(i16::max_value());
    assert_eq!(result, i16::max_value().to_string());
    assert_eq!(result, Buffer128::fmt_i16(i16::max_value()).as_str());

    let result = buffer.format_i16(i16::min_value());
    assert_eq!(result, i16::min_value().to_string());
    assert_eq!(result, Buffer128::fmt_i16(i16::min_value()).as_str());

    let result = buffer.format_i32(i32::max_value());
    assert_eq!(result, i32::max_value().to_string());
    assert_eq!(result, Buffer128::fmt_i32(i32::max_value()).as_str());

    let result = buffer.format_i32(i32::min_value());
    assert_eq!(result, i32::min_value().to_string());
    assert_eq!(result, Buffer128::fmt_i32(i32::min_value()).as_str());

    let result = buffer.format_i64(i64::max_value());
    assert_eq!(result, i64::max_value().to_string());
    assert_eq!(result, Buffer128::fmt_i64(i64::max_value()).as_str());

    let result = buffer.format_i64(i64::min_value());
    assert_eq!(result, i64::min_value().to_string());
    assert_eq!(result, Buffer128::fmt_i64(i64::min_value()).as_str());

    let result = buffer.format_isize(isize::max_value());
    assert_eq!(result, isize::max_value().to_string());
    assert_eq!(result, Buffer128::fmt_isize(isize::max_value()).as_str());

    let result = buffer.format_isize(isize::min_value());
    assert_eq!(result, isize::min_value().to_string());
    assert_eq!(result, Buffer128::fmt_isize(isize::min_value()).as_str());

    let result = buffer.format_i128(i128::max_value());
    assert_eq!(result, i128::max_value().to_string());
    assert_eq!(result, Buffer128::fmt_i128(i128::max_value()).as_str());
}

#[test]
fn should_convert_u8() {
    let mut expected = String::with_capacity(u8::TEXT_SIZE);
    let mut buffer = [0u8; u8::TEXT_SIZE];
    for num in [u8::min_value(), 10, 100, 150, u8::max_value()] {
        let _ = write!(&mut expected, "{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);

        if let Some(non_zero) = num::NonZeroU8::new(num) {
            assert_eq!(non_zero.to_str(&mut buffer), expected);
        }

        expected.clear();
    }
}

#[test]
fn should_convert_u16() {
    let mut expected = String::with_capacity(u16::TEXT_SIZE);
    let mut buffer = [0u8; u16::TEXT_SIZE];
    for num in [u16::min_value(), 10, 105, 10_500, 50_123, u16::max_value()] {
        let _ = write!(&mut expected, "{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);

        if let Some(non_zero) = num::NonZeroU16::new(num) {
            assert_eq!(non_zero.to_str(&mut buffer), expected);
        }

        expected.clear();
    }
}

#[test]
fn should_convert_u32() {
    let mut expected = String::with_capacity(u32::TEXT_SIZE);
    let mut buffer = [0u8; u32::TEXT_SIZE];
    for num in [u32::min_value(), 10, 105, 10_500, 50_123, 250_987, u32::max_value()] {
        let _ = write!(&mut expected, "{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);

        if let Some(non_zero) = num::NonZeroU32::new(num) {
            assert_eq!(non_zero.to_str(&mut buffer), expected);
        }

        expected.clear();
    }
}

#[test]
fn should_convert_u128() {
    let mut expected = String::with_capacity(u128::TEXT_SIZE);
    let mut buffer = [0u8; u128::TEXT_SIZE];
    let mut num = u128::max_value();
    loop {
        let _ = write!(&mut expected, "{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);

        if let Some(non_zero) = num::NonZeroU128::new(num) {
            assert_eq!(non_zero.to_str(&mut buffer), expected);
        }

        if num == 0 {
            break;
        }

        expected.clear();
        num /= u8::max_value() as u128;
    }
}

#[test]
fn should_convert_u128_without_missing_leading_zeros() {
    let mut expected = String::with_capacity(u128::TEXT_SIZE);
    let mut buffer = [0u8; u128::TEXT_SIZE];

    let inputs = [
        70000010000000100001u128,
        300000000000000000000000000000000000000u128,
    ];

    for input in inputs {
        let _ = write!(&mut expected, "{}", input);
        assert_eq!(expected, input.to_str(&mut buffer));

        if let Some(non_zero) = num::NonZeroU128::new(input) {
            assert_eq!(expected, non_zero.to_str(&mut buffer));
        }

        expected.clear();
    }
}

#[test]
fn should_convert_i8() {
    let mut expected = String::with_capacity(i8::TEXT_SIZE);
    let mut buffer = [0u8; i8::TEXT_SIZE];
    for num in [i8::min_value(), -11, 0, 11, i8::max_value()] {
        let _ = write!(&mut expected, "{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);

        if let Some(non_zero) = num::NonZeroI8::new(num) {
            assert_eq!(non_zero.to_str(&mut buffer), expected);
        }

        expected.clear()
    }
}

#[test]
fn should_convert_i16() {
    let mut expected = String::with_capacity(i16::TEXT_SIZE);
    let mut buffer = [0u8; i16::TEXT_SIZE];
    for num in [i16::min_value(), -5_100, -11, 0, 11, 5_100, i16::max_value()] {
        let _ = write!(&mut expected, "{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);

        if let Some(non_zero) = num::NonZeroI16::new(num) {
            assert_eq!(non_zero.to_str(&mut buffer), expected);
        }

        expected.clear()
    }
}

#[test]
fn should_convert_i32() {
    let mut expected = String::with_capacity(i32::TEXT_SIZE);
    let mut buffer = [0u8; i32::TEXT_SIZE];
    for num in [i32::min_value(), -250_765, -15_100, -11, 0, 11, 15_100, 250_765, i32::max_value()] {
        let _ = write!(&mut expected, "{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);

        if let Some(non_zero) = num::NonZeroI32::new(num) {
            assert_eq!(non_zero.to_str(&mut buffer), expected);
        }

        expected.clear()
    }
}

#[test]
fn should_convert_i128() {
    let mut expected = String::with_capacity(i128::TEXT_SIZE);
    let mut buffer = [0u8; i128::TEXT_SIZE];
    let mut num = i128::max_value();

    loop {
        let _ = write!(&mut expected, "{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);

        if let Some(non_zero) = num::NonZeroI128::new(num) {
            assert_eq!(non_zero.to_str(&mut buffer), expected);
        }

        expected.clear();

        if num == 0 {
            break;
        }

        num /= u8::max_value() as i128;
    }

    num = i128::min_value();

    loop {
        let _ = write!(&mut expected, "{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);

        if num == 0 {
            break;
        }

        num /= u8::max_value() as i128;
        expected.clear()
    }
}

#[test]
fn should_convert_ptr() {
    let mut expected = String::with_capacity(<*const u8>::TEXT_SIZE);
    let mut buffer = [0u8; <*const u8>::TEXT_SIZE];
    let mut num = usize::max_value();

    loop {
        let ptr = num as *const u8;
        let _ = write!(&mut expected, "{:p}", ptr);
        assert_eq!(ptr.to_str(&mut buffer), expected);

        if num == 0 {
            break;
        }

        num /= u8::max_value() as usize;
        expected.clear()
    }
}
