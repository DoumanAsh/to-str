use to_str::ToStr;

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
        num.to_str(&mut buffer);
        println!("buffer={:?}", buffer);
        assert_eq!(num.to_str(&mut buffer), expected);
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
