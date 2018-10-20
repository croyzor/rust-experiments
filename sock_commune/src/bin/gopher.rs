use std::io::prelude::{Read, Write};
use std::net::*;
use std::string::*;
use sock_commune::parse_uri;

fn print_usage(name: String) {
    println!("Usage:");
    println!("\t{} <DESTINATION>", name);
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        print_usage(args[0].parse().unwrap());
        return Ok(());
    }

    let raw_addr = &args[1];
    let hole = parse_uri(&raw_addr).unwrap();

    // TODO: Validate argument
    let mut stream = TcpStream::connect(hole.to_string())?;
    stream.write("\r\n".as_bytes())?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;
    let resp = String::from_utf8(buf).unwrap();
    println!("{}", resp);
    Ok(())
}

#[cfg(test)]
mod tests {
    use sock_commune::parse_uri;
    use sock_commune::gopher::*;

    #[test]
    fn parse_args() {
        assert_eq!(parse_uri("gopher://test.com:70"),
                   Ok(GopherHole {
                       url: "test.com".to_string(),
                       port: 70
                   }));
    }
}
