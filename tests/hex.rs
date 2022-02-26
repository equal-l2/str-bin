use str_bin;

#[test]
fn encode() {
    let buf = b"hello";
    let s = str_bin::to_hex_str(buf, false);
    assert_eq!(s, "68656c6c6f");
}

#[test]
fn decode() {
    let hoge = "686f6765";
    let s = str_bin::from_hex_str(&hoge, false);
    assert!(s.is_ok());
    let s = s.unwrap();
    assert_eq!(s, b"hoge");
}

#[test]
fn encode_reversed() {
    let buf = b"hello";
    let s = str_bin::to_hex_str(buf, true);
    assert_eq!(s, "8656c6c6f6");
}

#[test]
fn decode_reversed() {
    let hoge = "86f67656";
    let s = str_bin::from_hex_str(&hoge, true);
    assert!(s.is_ok());
    let s = s.unwrap();
    assert_eq!(s, b"hoge");
}

#[test]
fn invalid_length() {
    let hoge_cut = "8656c6c6f";
    let s = str_bin::from_hex_str(&hoge_cut, false);
    assert!(s.is_err());
    let e = s.unwrap_err();
    assert!(matches!(
        e,
        str_bin::Error::InvalidLength {
            len: 9,
            must_be_multiple_of: 2
        }
    ));
}

#[test]
fn invalid_char() {
    let hoge_add = "8656c6c6fg";
    let s = str_bin::from_hex_str(&hoge_add, false);
    assert!(s.is_err());
    let e = s.unwrap_err();
    assert!(matches!(e, str_bin::Error::InvalidChar('g')));
}
