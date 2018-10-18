#[macro_use]
extern crate nom;

use std::io::prelude::{Read, Write};
use std::net::*;
use std::string::*;
use std::str::FromStr;

fn print_usage(name: String) {
    println!("Usage:");
    println!("\t{} <DESTINATION>", name);
}

struct GopherHole {
    url: String,
    port: u64,
}

impl GopherHole {
    fn to_url(input: &str) -> Result<String, ()> {
        Ok(input.to_string())
    }

    fn to_port(input: &str) -> Result<u64, std::num::ParseIntError> {
        u64::from_str(input)
    }

    fn to_string(self) -> String {
        format!("{}:{}", self.url, self.port)
    }
}
named!(parse_uri<&str, GopherHole>,
       do_parse!(
           opt!(tag!("gopher://")) >>
               // TODO: Handle addresses with no port
               url:  map_res!(take_until!(":"), GopherHole::to_url) >>
               // TODO: Actually read the port
               port: map_res!(value!("70"), GopherHole::to_port) >>
       (GopherHole { url, port })));


fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        print_usage(args[0].parse().unwrap());
        return Ok(());
    }

    let raw_addr = &args[1];
    let (_, hole) = parse_uri(&raw_addr).unwrap();

    // TODO: Validate argument
    let mut stream = TcpStream::connect(hole.to_string())?;
    stream.write("\r\n".as_bytes())?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;
    let resp = String::from_utf8(buf).unwrap();
    println!("{}", resp);
    Ok(())
}
