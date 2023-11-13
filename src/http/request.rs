use std::collections::HashMap;

use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    OPTIONS,
    DELETE,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "PATCH" => Method::PATCH,
            "OPTIONS" => Method::OPTIONS,
            "DELETE" => Method::DELETE,
            _ => panic!("Unsupported HTTP method: {}", s),
        }
    }
}

// impl From<&str> for Method {
//     fn from(s: &str) -> Self {
//         println!("ini method -> {}", s);
//         if s == "GET" {
//             Method::GET
//         } else if s == "POST" {
//             Method::POST
//         } else {
//             Method::GET
//         }
//     }
// }

pub type RequestParseResult = Result<Request, Error>;

pub enum Error {
    ParsingError,
    IOError(std::io::Error),
    Utf8Error(std::string::FromUtf8Error),
}

impl From<std::io::Error> for Error {
    fn from(internal_err: std::io::Error) -> Self {
        Error::IOError(internal_err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(internal_err: std::string::FromUtf8Error) -> Self {
        Error::Utf8Error(internal_err)
    }
}

#[derive(Debug)]

pub enum Version {
    HTTP1_1,
    HTTP2,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::HTTP1_1,
            "HTTP/2" => Version::HTTP2,
            _ => panic!("Unsupported HTTP version: {}", s),
        }
    }
}

pub struct Request {
    pub method: Method,
    pub uri: String,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub path_params: HashMap<String, String>,
}

impl Request {
    pub async fn new(reader: &mut tokio::net::TcpStream) -> RequestParseResult {
        let mut first_line: String = String::new();

        let mut headers: HashMap<String, String> = HashMap::new();

        let mut buffer: Vec<u8> = std::vec::Vec::new();

        loop {
            let b = reader.read_u8().await?;
            buffer.push(b);
            if b as char == '\n' {
                if first_line.is_empty() {
                    // membaca line pertama pada buffer content
                    first_line = String::from_utf8(buffer[0..buffer.len() - 2].to_vec())?;
                    buffer.clear();
                } else {
                    if buffer.len() == 2 && buffer[0] as char == '\r' {
                        break;
                    }
                    // asumsinya semua yang berada pada block ini adalah header
                    let header_line = String::from_utf8(buffer[0..buffer.len() - 2].to_vec())?;

                    buffer.clear();
                    let mut iter = header_line.split(":");
                    // iter.next().unwrap(); // unwrap bisa membuat program panic

                    let key = match iter.next() {
                        // lebih aman menggunakan fitur match pattern
                        Some(k) => k,
                        None => return Err(Error::ParsingError),
                    };

                    let value = match iter.next() {
                        // lebih aman menggunakan fitur match pattern
                        Some(k) => k,
                        None => return Err(Error::ParsingError),
                    };

                    // push kedalam header
                    headers.insert(key.to_string(), value.to_string());
                }
            }
        }
        let mut first_line_iter = first_line.split(" ");

        let method: Method = first_line_iter.next().unwrap().into();

        let uri_iter_next_unwrap = first_line_iter.next().unwrap().to_string();

        let mut uri_iter = uri_iter_next_unwrap.split("?");

        let uri = match uri_iter.next() {
            Some(u) => u,
            None => return Err(Error::ParsingError),
        }; // -> params?ucok=subara

        // kondisi url dengan params
        let mut query_params: HashMap<String, String> = HashMap::new();
        match uri_iter.next() {
            Some(q) => {
                for kv in q.split("&") {
                    let mut iter = kv.split("=");
                    let key = match iter.next() {
                        Some(k) => k,
                        None => return Err(Error::ParsingError),
                    };

                    let value = match iter.next() {
                        Some(k) => k,
                        None => return Err(Error::ParsingError),
                    };

                    query_params.insert(key.to_string(), value.to_string());
                }
            }
            None => (),
        }

        Ok(Request {
            method,
            uri: uri.to_string(),
            version: first_line_iter.next().unwrap().into(),
            headers,
            query_params,
            path_params: HashMap::new(),
        })
    }
}

pub struct Response {}

pub struct Connection {
    pub request: Request,
    pub socket: tokio::net::TcpStream,
}

impl Connection {
    pub async fn new(mut socket: tokio::net::TcpStream) -> Result<Connection, Error> {
        let request = Request::new(&mut socket).await?;
        // let response = Response {};
        Ok(Connection { request, socket })
    }

    pub async fn respond(&self, status: usize, body: &str) -> Result<(), Error> {
        Ok(())
    }
}
