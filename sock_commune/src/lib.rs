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

named!(_parse_line_resource<&str, Link>,
       do_parse!(
           linktype: map_res!(take!(1), parse_link_type) >>
               disp: take_until!("\t") >>
               char!('\t') >>
               selector: take_until!("\t") >>
               char!('\t') >>
               host: take_until!("\t") >>
               char!('\t') >>
               port: map_res!(rest, Link::to_port) >>
               (Link {
                   name: disp.to_string(),
                   url: host.to_string(),
                   port: port,
                   selector: selector.to_string(),
                   what: linktype,
               })));

pub fn parse_line_resource(line: &str) -> Result<Link,
                                                nom::Err<&str, u32>> {
    _parse_line_resource(line).map(|(_, res)| res)
}

fn parse_link_type(identifier: &str) -> Result<LinkType, String> {
    match identifier {
        "i" => Ok(LinkType::Info),
        "1" => Ok(LinkType::Folder),
        "0" => Ok(LinkType::Text),
        _   => Err("Link type not implemented".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

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

    #[test]
    fn parse_type() {
        assert_eq!(parse_link_type("1"), Ok(LinkType::Folder));
    }

    #[test]
    fn parse_line() {
        // From wikipedia
        let line = format!("{}	{}	{}	{}",
                           "1Floodgap Home",
                           "/home",
                           "gopher.floodgap.com",
                           70);
        assert_eq!(parse_line_resource(&line),
                   Ok(Link {
                       name: "Floodgap Home".to_string(),
                       url: "gopher.floodgap.com".to_string(),
                       port: 70,
                       selector: "/home".to_string(),
                       what: LinkType::Folder,
                   }));
    }
}
