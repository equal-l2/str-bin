fn bin_to_str(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 8);
    for b in bytes {
        write!(s, "{:08b}", b).unwrap();
    }
    s
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("The input length ({0}) is not a multiple of 8")]
    InvalidLength(usize),
    #[error("Invalid character \"{0}\" is found")]
    InvalidChar(char),
}

fn str_to_bin(s: &str) -> Result<Vec<u8>, Error> {
    if s.len() % 8 != 0 {
        return Err(Error::InvalidLength(s.len()));
    }
    let mut bytes = vec![];
    let mut it = s.chars();

    {
        'outer: loop {
            let mut byte: u8 = 0;
            let mut mask = 1 << 7;
            for _ in 0..8 {
                let ch = it.next();
                if let Some(c) = ch {
                    match c {
                        '0' => {}
                        '1' => {
                            byte |= mask;
                        }
                        _ => return Err(Error::InvalidChar(c)),
                    }
                    mask >>= 1;
                } else {
                    // as s.len() % 8 == 0, it.next() == None means it reaches end of str
                    break 'outer;
                }
            }
            bytes.push(byte);
        }
    }

    Ok(bytes)
}

#[derive(structopt::StructOpt)]
enum Opt {
    /// read binary from stdin and write 0/1 string to stdout
    Encode {},
    /// read 0/1 string from stdin and write binary to stdout
    Decode {},
}

fn main() {
    use std::io::Read;
    use structopt::StructOpt;
    let opt = Opt::from_args();

    match opt {
        Opt::Encode {} => {
            let mut buf = vec![];
            std::io::stdin().read_to_end(&mut buf).unwrap();
            let s = bin_to_str(&buf);
            print!("{}", s);
        }
        Opt::Decode {} => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf).unwrap();
            let s = buf.trim();
            let res = str_to_bin(s);
            match res {
                Ok(bin) => {
                    use std::io::Write;
                    std::io::stdout().write_all(&bin).unwrap();
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode() {
        let buf = b"hello";
        let s = super::bin_to_str(buf);
        assert_eq!(s, "0110100001100101011011000110110001101111");
    }

    #[test]
    fn test_decode() {
        let hoge = "01101000011011110110011101100101";
        let s = super::str_to_bin(&hoge);
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, b"hoge");
    }

    #[test]
    fn test_invalid_length() {
        let hoge_cut = "011010000110111101100111011001";
        let s = super::str_to_bin(&hoge_cut);
        assert!(s.is_err());
        let e = s.unwrap_err();
        assert!(matches!(e, super::Error::InvalidLength(30)));
    }

    #[test]
    fn test_invalid_char() {
        let hoge_add = "01101000011011110110011101100102";
        let s = super::str_to_bin(&hoge_add);
        assert!(s.is_err());
        let e = s.unwrap_err();
        assert!(matches!(e, super::Error::InvalidChar('2')));
    }
}
