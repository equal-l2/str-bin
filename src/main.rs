#[derive(clap::StructOpt)]
enum SubCommand {
    /// read data from stdin and write obfuscated string to stdout
    Encode,
    /// read obfuscated string from stdin and write data to stdout
    Decode,
}

#[derive(clap::StructOpt)]
struct Opt {
    #[clap(subcommand)]
    subcommand: SubCommand,

    #[clap(short, long)]
    reverse: bool,

    #[clap(long, short)]
    hex: bool,
}

fn main() {
    use std::io::Read;
    let opt: Opt = clap::Parser::parse();

    match opt.subcommand {
        SubCommand::Encode {} => {
            let mut buf = vec![];
            std::io::stdin().read_to_end(&mut buf).unwrap();
            let s = if opt.hex {
                str_bin::to_hex_str(&buf, opt.reverse)
            } else {
                str_bin::to_bin_str(&buf, opt.reverse)
            };
            print!("{}", s);
        }
        SubCommand::Decode {} => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf).unwrap();
            let s = buf.trim();
            let res = if opt.hex {
                str_bin::from_hex_str(s, opt.reverse)
            } else {
                str_bin::from_bin_str(s, opt.reverse)
            };
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
}
