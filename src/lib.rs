use std::{collections::HashMap, str};
use thiserror::Error;

mod ascii;
use ascii::{string_from_ascii, NonAsciiCharacterError};
mod status;
pub use status::Status;

#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub target: String,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("no request line")]
    NoRequestLine,
    #[error("invalid request line")]
    InvalidRequestLine,
    #[error("non ASCII character")]
    NotAsciiEncoding(#[from] NonAsciiCharacterError),
    #[error("invalid http method")]
    InvalidMethod(#[from] InvalidMethodError),
    #[error("invalid http version")]
    InvalidVersion(#[from] InvalidVersionError),
    #[error("no message end")]
    NoEnd,
    #[error("no colon in header")]
    NoHeaderColon,
}

impl Request {
    pub fn from(v: &[u8]) -> Result<Request, RequestError> {
        let mut lines = v
            .split(|&b| b == b'\n')
            .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
            .peekable();

        let mut start_line = lines
            .next()
            .ok_or(RequestError::InvalidRequestLine)?
            .split(|&b| b == b' ');

        Ok(Request {
            method: Method::try_from(&string_from_ascii(
                start_line.next().ok_or(RequestError::InvalidRequestLine)?,
            )?)?,
            target: string_from_ascii(start_line.next().ok_or(RequestError::InvalidRequestLine)?)?,
            version: Version::try_from(&string_from_ascii(
                start_line.next().ok_or(RequestError::InvalidRequestLine)?,
            )?)?,
            headers: {
                let mut headers = HashMap::<String, String>::new();
                // while double CRLF is not reached
                while lines.peek().is_some_and(|b| b != b"") {
                    let line = lines.next().unwrap();
                    let index = line
                        .iter()
                        .position(|&b| b == b':')
                        .ok_or(RequestError::NoHeaderColon)?;
                    let (k, mut v) = line.split_at(index);

                    // skip ':'
                    v = &v[1..];

                    let from = v.iter().position(|b| !b.is_ascii_whitespace()).unwrap_or(0);
                    let to = v
                        .iter()
                        .rposition(|b| !b.is_ascii_whitespace())
                        .unwrap_or(0);
                    v = &v[from..=to];

                    headers.insert(string_from_ascii(k)?, string_from_ascii(v)?);
                }
                lines.next().ok_or(RequestError::NoEnd)?;

                headers
            },
            body: lines.next().ok_or(RequestError::NoEnd)?.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_from() {
        assert_eq!(
            Request::from(format!("GET /index.html HTTP/1.1\r\n\r\n\r\n\r\n").as_bytes()).unwrap(),
            Request {
                method: Method::Get,
                target: String::from("/index.html"),
                version: Version::OneOne,
                headers: HashMap::new(),
                body: Vec::new()
            }
        )
    }
}

pub struct Response {
    pub version: Version,
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn to_bytes(self) -> Vec<u8> {
        let mut v = Vec::<u8>::from(format!(
            "{} {} {}\r\n\r\n{}\r\n\r\n",
            self.version.to_string(),
            self.status.code(),
            self.status.text(),
            {
                let mut h = self
                    .headers
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}\r\n\r\n"))
                    .collect::<Vec<String>>();
                h.sort();
                h.join("")
            }
        ));
        v.extend_from_slice(&self.body);
        v
    }

    pub fn from_status(status: Status) -> Self {
        Response {
            version: Version::OneOne,
            status: status,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
}

#[derive(Error, Debug)]
#[error("invalid http method")]
pub struct InvalidMethodError;

impl Method {
    pub fn try_from(s: &str) -> Result<Self, InvalidMethodError> {
        match s {
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            _ => Err(InvalidMethodError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Version {
    OneOne,
    Two,   // not implemented
    Three, // not implemented
}

#[derive(Error, Debug)]
#[error("invalid http version")]
pub struct InvalidVersionError;

impl Version {
    pub fn to_string(self) -> String {
        match self {
            Self::OneOne => String::from("HTTP/1.1"),
            Self::Two => String::from("HTTP/2.0"),
            Self::Three => String::from("HTTP/3.0"),
        }
    }

    pub fn try_from(s: &str) -> Result<Self, InvalidVersionError> {
        match s {
            "HTTP/1.1" => Ok(Self::OneOne),
            "HTTP/2.0" => Ok(Self::Two),
            "HTTP/3.0" => Ok(Self::Three),
            _ => Err(InvalidVersionError),
        }
    }
}
