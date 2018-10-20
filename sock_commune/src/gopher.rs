use std::str::FromStr;

#[derive(PartialEq,Debug)]
pub struct GopherHole {
    pub url: String,
    pub port: u64,
}

impl GopherHole {
    pub fn to_url(input: &str) -> Result<String, ()> {
        Ok(input.to_string())
    }

    pub fn to_port(input: &str) -> Result<u64, std::num::ParseIntError> {
        u64::from_str(input)
    }

    pub fn to_string(self) -> String {
        format!("{}:{}", self.url, self.port)
    }
}
