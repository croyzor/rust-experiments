#[macro_use]
extern crate nom;
pub mod gopher;

use self::gopher::GopherHole;

named!(_parse_uri<&str, GopherHole>,
       do_parse!(
           opt!(tag!("gopher://")) >>
               // TODO: Handle addresses with no port
               url:  map_res!(alt!(take_until!(":")
                                   | take_until!("\n"))
                              , GopherHole::to_url) >>
               // TODO: Actually read the port
               port: map_res!(value!("70"), GopherHole::to_port) >>
               (GopherHole { url, port })));

// TODO: Terrible error type!
pub fn parse_uri(uri: &str) -> Result<GopherHole, ()> {
    match _parse_uri(uri) {
        Ok((_, hole)) => Ok(hole),
        Err(_) => Err(()),
    }
}
