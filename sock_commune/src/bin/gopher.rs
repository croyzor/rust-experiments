use std::io::prelude::{Read, Write};
use std::net::TcpStream;
use sock_commune::parse_uri;
use sock_commune::gopher::*;

use std::io;

fn print_usage(name: String) {
    println!("Usage:");
    println!("\t{} <DESTINATION>", name);
}

fn fetch_link(link: &Link) -> io::Result<String> {

    let mut stream = TcpStream::connect(link.to_string())?;
    stream.write(format!("{}\r\n", link.selector).as_bytes())?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;
    match String::from_utf8(buf) {
        Ok(resp) => {
            println!("{}", resp);
            Ok(resp.to_string())
        }
        Err(_) =>
            Err(io::Error::new(io::ErrorKind::Other,
                               "Failed to read server response as text"))
    }
}


fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        print_usage(args[0].parse().unwrap());
        return Ok(());
    }

    let raw_addr = &args[1];
    let gopher_hole = parse_uri(&raw_addr).unwrap();

    fetch_link(&gopher_hole).map(|_| ())
}
