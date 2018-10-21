#[macro_use]
extern crate nom;
pub mod gopher;

use nom::rest;
use self::gopher::{GopherHole,Link};

// nom parser for uris of the format `gopher://test.com:70`
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
            selector: String::new(),
        }),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use crate::{GopherHole,parse_uri};

    // Here are a few tests to exercise the parse_uri function, which will
    // be used to parse the url argument passed to the gopher client. It
    // may be used in future to parse links returned by querying gopher
    // holes too...

    #[test]
    fn parse_args_with_port() {
        assert_eq!(parse_uri("gopher://test.com:70"),
                   Ok(GopherHole {
                       url: "test.com".to_string(),
                       port: 70
                   }));
    }

    #[test]
    fn parse_args_without_prefix() {
        assert_eq!(parse_uri("test.com:70"),
                   Ok(GopherHole {
                       url: "test.com".to_string(),
                       port: 70
                   }));
    }

    #[test]
    fn parse_args() {
        assert_eq!(parse_uri("gopher://test.com"),
                   Ok(GopherHole {
                       url: "test.com".to_string(),
                       port: 70
                   }));
    }
}
