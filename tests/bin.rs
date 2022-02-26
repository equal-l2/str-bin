use str_bin;

#[test]
fn encode() {
    let buf = b"hello";
    let s = str_bin::to_bin_str(buf, false);
    assert_eq!(s, "0110100001100101011011000110110001101111");
}

#[test]
fn decode() {
    let hoge = "01101000011011110110011101100101";
    let s = str_bin::from_bin_str(&hoge, false);
    assert!(s.is_ok());
    let s = s.unwrap();
    assert_eq!(s, b"hoge");
}

#[test]
fn encode_reversed() {
    let buf = b"hello";
    let s = str_bin::to_bin_str(buf, true);
    assert_eq!(s, "0001011010100110001101100011011011110110");
}

#[test]
fn decode_reversed() {
    let hoge = "00010110111101101110011010100110";
    let s = str_bin::from_bin_str(&hoge, true);
    assert!(s.is_ok());
    let s = s.unwrap();
    assert_eq!(s, b"hoge");
}

#[test]
fn invalid_length() {
    let hoge_cut = "011010000110111101100111011001";
    let s = str_bin::from_bin_str(&hoge_cut, false);
    assert!(s.is_err());
    let e = s.unwrap_err();
    assert!(matches!(
        e,
        str_bin::Error::InvalidLength {
            len: 30,
            must_be_multiple_of: 8
        }
    ));
}

#[test]
fn invalid_char() {
    let hoge_add = "01101000011011110110011101100102";
    let s = str_bin::from_bin_str(&hoge_add, false);
    assert!(s.is_err());
    let e = s.unwrap_err();
    assert!(matches!(e, str_bin::Error::InvalidChar('2')));
}
