use std::str::FromStr;

// Kinds of things which can be returned by querying a gopher hole.
// Will be fleshed out more as I implement features.
#[derive(PartialEq,Debug)]
pub enum LinkType {
    Text,
    Folder,
    Info,
}

#[derive(PartialEq,Debug)]
pub struct Link {
    pub name: String,
    pub url: String,
    pub port: u64,
    pub selector: String,
    pub what: LinkType,
}

impl Link {
    pub fn new(url: String, port: u64) -> Link {
        Link {
            name: String::new(),
            url: url,
            port: port,
            selector: String::new(),
            what: LinkType::Folder,
        }
    }

    // This method needs to return a Result to be used with
    // `map_res` in a nom parser
    pub fn to_url(input: &str) -> Result<String, ()> {
        Ok(input.to_string())
    }

    // Parse a string as a port number (just a u64)
    pub fn to_port(input: &str) -> Result<u64, std::num::ParseIntError> {
        u64::from_str(input)
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.url, self.port)
    }
}

