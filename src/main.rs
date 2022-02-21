fn bin_to_str(bytes: &[u8], reversed: bool) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 8);
    for b in bytes {
        let out = if reversed { b.reverse_bits() } else { *b };
        write!(s, "{:08b}", out).unwrap();
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

fn str_to_bin(s: &str, reversed: bool) -> Result<Vec<u8>, Error> {
    let len = s.len();
    if len % 8 != 0 {
        return Err(Error::InvalidLength(len));
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

#[derive(clap::StructOpt)]
enum SubCommand {
    /// read binary from stdin and write 0/1 string to stdout
    Encode {},
    /// read 0/1 string from stdin and write binary to stdout
    Decode {},
}

#[derive(clap::StructOpt)]
struct Opt {
    #[clap(subcommand)]
    subcommand: SubCommand,
    /// bitwise reverse
    #[clap(short, long)]
    reverse: bool,
}

fn main() {
    use std::io::Read;
    let opt: Opt = clap::Parser::parse();

    match opt.subcommand {
        SubCommand::Encode {} => {
            let mut buf = vec![];
            std::io::stdin().read_to_end(&mut buf).unwrap();
            let s = bin_to_str(&buf, opt.reverse);
            print!("{}", s);
        }
        SubCommand::Decode {} => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf).unwrap();
            let s = buf.trim();
            let res = str_to_bin(s, opt.reverse);
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
    fn verify_app() {
        use clap::CommandFactory;
        super::Opt::command().debug_assert()
    }

    #[test]
    fn test_encode() {
        let buf = b"hello";
        let s = super::bin_to_str(buf, false);
        assert_eq!(s, "0110100001100101011011000110110001101111");
    }

    #[test]
    fn test_encode_reversed() {
        let buf = b"hello";
        let s = super::bin_to_str(buf, true);
        assert_eq!(s, "0001011010100110001101100011011011110110");
    }

    #[test]
    fn test_decode_reversed() {
        let hoge = "00010110111101101110011010100110";
        let s = super::str_to_bin(&hoge, true);
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, b"hoge");
    }

    #[test]
    fn test_decode() {
        let hoge = "01101000011011110110011101100101";
        let s = super::str_to_bin(&hoge, false);
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, b"hoge");
    }

    #[test]
    fn test_invalid_length() {
        let hoge_cut = "011010000110111101100111011001";
        let s = super::str_to_bin(&hoge_cut, false);
        assert!(s.is_err());
        let e = s.unwrap_err();
        assert!(matches!(e, super::Error::InvalidLength(30)));
    }

    #[test]
    fn test_invalid_char() {
        let hoge_add = "01101000011011110110011101100102";
        let s = super::str_to_bin(&hoge_add, false);
        assert!(s.is_err());
        let e = s.unwrap_err();
        assert!(matches!(e, super::Error::InvalidChar('2')));
    }
}
