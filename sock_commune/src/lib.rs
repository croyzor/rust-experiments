#[macro_use]
extern crate nom;
pub mod gopher;

use nom::rest;
use self::gopher::*;

// nom parser for uris of the format `gopher://test.com:70`
named!(_parse_uri<&str, Link>,
    do_parse!(opt!(tag!("gopher://")) >>
              url: map_res!(alt_complete!(take_until!(":") |
                                         rest),
                            Link::to_url) >>
              // TODO: Read the port, if `take_until(":")` succeeds
              port: map_res!(value!("70"),
                             Link::to_port) >>
              (Link::new(url, port))));

pub fn parse_uri(uri: &str) -> Result<Link, nom::Err<&str, u32>> {
    _parse_uri(uri).map(|(_, res)| res)
}

named!(parse_line_info<&str, String>,
       do_parse!(tag!("i") >>
                 info: rest >>
                 (info.to_string())));

fn parse_linktype(identifier: &str) -> Result<LinkType, String> {
    match identifier {
        "i" => Ok(LinkType::Info),
        "1" => Ok(LinkType::Folder),
        "0" => Ok(LinkType::Text),
        _   => Err("Link type not implemented".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{Link,parse_uri};

    // Here are a few tests to exercise the parse_uri function, which will
    // be used to parse the url argument passed to the gopher client. It
    // may be used in future to parse links returned by querying gopher
    // holes too...

    #[test]
    fn parse_args_with_port() {
        assert_eq!(parse_uri("gopher://test.com:70"),
                   Ok(Link::new("test.com".to_string(),
                                70)));
    }

    #[test]
    fn parse_args_without_prefix() {
        assert_eq!(parse_uri("test.com:70"),
                   Ok(Link::new("test.com".to_string(),
                                70)));
    }

    #[test]
    fn parse_args() {
        assert_eq!(parse_uri("gopher://test.com"),
                   Ok(Link::new("test.com".to_string(),
                                70)));
    }
}
