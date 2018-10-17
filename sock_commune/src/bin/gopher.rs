use std::io::prelude::{Read, Write};
use std::net::*;
use std::string::*;

fn print_usage(name: String) {
    println!("Usage:");
    println!("\t{} <DESTINATION>", name);
}

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    let url;
    if args.len() != 2 {
        print_usage(args.next().unwrap());
        return Ok(());
    }
    else {
        // TODO: parse arguments nicely
        args.next();
        url = args.next();
    }

    // TODO: Validate argument
    let mut buf = [0; 512];
    let mut stream = TcpStream::connect(url.unwrap())?;
    stream.write("\r\n".as_bytes());
    stream.read(&mut buf);
    stream.read(&mut buf);
    let resp = String::from_utf8(buf.to_vec()).unwrap();
    println!("{}", resp);
    Ok(())
}
