use to_str::{ToStr, Buffer128};

#[test]
fn should_check_const_foramt() {
    let mut buffer = Buffer128::new();

    let result = buffer.format_u8(u8::max_value());
    assert_eq!(result, "255");

    let result = buffer.format_u16(u16::max_value());
    assert_eq!(result, u16::max_value().to_string());

    let result = buffer.format_u32(u32::max_value());
    assert_eq!(result, u32::max_value().to_string());

    let result = buffer.format_u64(u64::max_value());
    assert_eq!(result, u64::max_value().to_string());

    let result = buffer.format_u128(u128::max_value());
    assert_eq!(result, u128::max_value().to_string());
}

#[test]
fn should_convert_u8() {
    let mut buffer = [0u8; u8::TEXT_SIZE];
    for num in u8::min_value()..=u8::max_value() {
        let expected = format!("{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);
    }
}

#[test]
fn should_convert_u16() {
    let mut buffer = [0u8; u16::TEXT_SIZE];
    for num in u16::min_value()..=u16::max_value() {
        let expected = format!("{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);
    }
}

#[test]
fn should_convert_u128() {
    let mut buffer = [0u8; u128::TEXT_SIZE];
    let mut num = u128::max_value();
    loop {
        let expected = format!("{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);

        if num == 0 {
            break;
        }

        num /= u8::max_value() as u128;
    }
}

#[test]
fn should_convert_u128_without_missing_leading_zeros() {
    let mut buffer = [0u8; u128::TEXT_SIZE];

    let inputs = [
        70000010000000100001u128,
        300000000000000000000000000000000000000u128,
    ];

    for input in inputs {
        assert_eq!(input.to_string(), input.to_str(&mut buffer));
    }
}

#[test]
fn should_convert_i8() {
    let mut buffer = [0u8; i8::TEXT_SIZE];
    for num in i8::min_value()..=i8::max_value() {
        let expected = format!("{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);
    }
}

#[test]
fn should_convert_i16() {
    let mut buffer = [0u8; i16::TEXT_SIZE];
    for num in i16::min_value()..=i16::max_value() {
        let expected = format!("{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);
    }
}

#[test]
fn should_convert_i128() {
    let mut buffer = [0u8; u128::TEXT_SIZE];
    let mut num = u128::max_value();

    loop {
        let expected = format!("{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);

        if num == 0 {
            break;
        }

        num /= u8::max_value() as u128;
    }

    let mut num = u128::min_value();

    loop {
        let expected = format!("{}", num);
        assert_eq!(num.to_str(&mut buffer), expected);

        if num == 0 {
            break;
        }

        num /= u8::max_value() as u128;
    }
}

#[test]
fn should_convert_ptr() {
    let mut buffer = [0u8; <*const u8>::TEXT_SIZE];
    let mut num = usize::max_value();

    loop {
        let ptr = num as *const u8;
        let expected = format!("{:p}", ptr);
        assert_eq!(ptr.to_str(&mut buffer), expected);

        if num == 0 {
            break;
        }

        num /= u8::max_value() as usize;
    }
}
