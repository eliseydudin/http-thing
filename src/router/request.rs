use {
    httparse::{Request as HttpRequest, EMPTY_HEADER},
    std::{
        collections::HashMap,
        io::Read,
        net::{SocketAddr, TcpStream},
    },
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RequestType {
    Get,
    Post,
}

impl<'a> TryFrom<&'a str> for RequestType {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            _ => Err("Unknown request method".to_owned()),
        }
    }
}

pub struct Request {
    pub rtype: RequestType,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub data: Vec<u8>,
    pub addr: SocketAddr,
    pub fullpath: String,
    pub query: String,
}

const BUFFER_SIZE: usize = 8 * 1024;

impl Request {
    pub(crate) fn new(stream: &mut TcpStream, addr: SocketAddr) -> Result<Self, String> {
        let mut buffer = [0; BUFFER_SIZE];
        let size = match stream.read(&mut buffer) {
            Ok(d) => d,
            Err(e) => return Err(format!("{e}")),
        };
        let buffer = &buffer[..size];

        let mut headers = [EMPTY_HEADER; 100];
        let mut request = HttpRequest::new(&mut headers);
        let byte_offset = match request.parse(buffer) {
            Ok(size) => size,
            Err(e) => return Err(format!("{e}")),
        };

        let rtype = RequestType::try_from(request.method.ok_or("no method".to_owned())?)?;
        let path = request.path.ok_or("unknown")?.to_owned();
        let path = match path.find('?') {
            Some(pos) => path.chars().take(pos).collect(),
            None => path,
        };

        let fullpath = request.path.ok_or("no path".to_owned())?.to_owned();
        let query = (request.path.ok_or("no path".to_owned())?.to_owned())
            [fullpath.find("?").unwrap_or(fullpath.len())..]
            .to_string();

        let mut headers = HashMap::new();

        request.headers.to_vec().iter().for_each(|header| {
            headers.insert(
                header.name.to_owned(),
                String::from_utf8_lossy(header.value).to_string(),
            );
        });

        let byte_offset = match byte_offset {
            httparse::Status::Complete(data) => data,
            _ => return Err("byte_offset wasn't complete".to_owned()),
        };

        let data = buffer[byte_offset..].to_vec();

        Ok(Self {
            rtype,
            path,
            headers,
            data,
            addr,
            fullpath,
            query,
        })
    }
}
