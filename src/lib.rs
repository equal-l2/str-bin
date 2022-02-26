/// Converts bytes to 0/1 string
///
/// example:  `b"h"` → `"01101000"`  
/// reversed: `b"h"` → `"00010110"`  
pub fn to_bin_str(bytes: &[u8], reversed: bool) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 8);
    for b in bytes {
        let out = if reversed { b.reverse_bits() } else { *b };
        write!(s, "{:08b}", out).unwrap();
    }
    s
}

/// Converts bytes to hex string
///
/// example:  `b"h"` → `"68"`  
/// reversed: `b"h"` → `"86"`  
pub fn to_hex_str(bytes: &[u8], reversed: bool) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let out = if reversed {
            let upper = b & 0xF0;
            let lower = b & 0x0F;
            (lower << 4) | (upper >> 4)
        } else {
            *b
        };
        write!(s, "{:02x}", out).unwrap();
    }
    s
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The input length ({len}) is not a multiple of {must_be_multiple_of}")]
    InvalidLength {
        len: usize,
        must_be_multiple_of: usize,
    },
    #[error("Invalid character \"{0}\" is found")]
    InvalidChar(char),
}

/// Converts 0/1 string to bytes
///
/// example: `"01101000"` → `b"h"`
pub fn from_bin_str(s: &str, reversed: bool) -> Result<Vec<u8>, Error> {
    let len = s.len();
    if len % 8 != 0 {
        return Err(Error::InvalidLength {
            len,
            must_be_multiple_of: 8,
        });
    }

    let mut bytes = Vec::with_capacity(len / 8);
    {
        let mut it = s.chars().peekable();
        loop {
            if it.peek().is_some() {
                let mut byte: u8 = 0;
                let mut mask = 1 << 7;
                for _ in 0..8 {
                    let c = it
                        .next()
                        .unwrap_or_else(|| unreachable!("length is multiple of 8"));
                    match c {
                        '0' => {}
                        '1' => byte |= mask,
                        _ => return Err(Error::InvalidChar(c)),
                    }
                    mask >>= 1;
                }
                let out = if reversed { byte.reverse_bits() } else { byte };
                bytes.push(out);
            } else {
                break;
            }
        }
    }

    Ok(bytes)
}

const fn is_valid_hex_char(c: char) -> bool {
    ('a' <= c && c <= 'f') || ('0' <= c && c <= '9')
}

const fn parse_hex_digit(c: char) -> u8 {
    assert!(is_valid_hex_char(c));
    let c = c as u8;
    if b'a' <= c && c <= b'f' {
        // a ~ f (10 ~ 15)
        return c - b'a' + 10;
    } else {
        // 0 ~ 9
        return c - b'0';
    }
}

/// Converts hex string to bytes
///
/// example: `"68"` → `b"h"`
pub fn from_hex_str(s: &str, reversed: bool) -> Result<Vec<u8>, Error> {
    let len = s.len();
    if len % 2 != 0 {
        return Err(Error::InvalidLength {
            len,
            must_be_multiple_of: 2,
        });
    }

    let mut bytes = Vec::with_capacity(len / 2);
    {
        let mut it = s.chars().peekable();
        loop {
            if it.peek().is_some() {
                let c1 = it
                    .next()
                    .unwrap_or_else(|| unreachable!("an element is guaranteed to exist"));
                if !is_valid_hex_char(c1) {
                    return Err(Error::InvalidChar(c1));
                }

                let c2 = it
                    .next()
                    .unwrap_or_else(|| unreachable!("length is multiple of 2"));
                if !is_valid_hex_char(c2) {
                    return Err(Error::InvalidChar(c2));
                }

                let upper = parse_hex_digit(c1);
                let lower = parse_hex_digit(c2);
                let out = if reversed {
                    (lower << 4) | upper
                } else {
                    (upper << 4) | lower
                };
                bytes.push(out);
            } else {
                break;
            }
        }
    }

    Ok(bytes)
}
