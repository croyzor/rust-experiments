#[macro_use]
extern crate nom;
pub mod gopher;

use nom::rest;
use self::gopher::GopherHole;

named!(_parse_uri<&str, (String, u64)>,
    do_parse!(opt!(tag!("gopher://")) >>
              res: tuple!(
                  map_res!(alt_complete!(take_until!(":") |
                                         rest),
                           GopherHole::to_url),
                  // TODO: Read the port, if `take_until(":")` succeeds
                  map_res!(value!("70"),
                           GopherHole::to_port)) >>
              (res)));

pub fn parse_uri(uri: &str) -> Result<GopherHole, nom::Err<&str, u32>> {
    match _parse_uri(uri) {
        Ok((_, (a,b))) => Ok(GopherHole {
            url: a,
            port: b,
        }),
        Err(e) => Err(e)
    }
}
