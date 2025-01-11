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
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            _ => Err(()),
        }
    }
}

pub struct Request {
    pub rtype: RequestType,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub data: Vec<u8>,
    pub addr: SocketAddr,
}

const BUFFER_SIZE: usize = 8 * 1024;

impl Request {
    pub(crate) fn new(stream: &mut TcpStream, addr: SocketAddr) -> Option<Self> {
        let mut buffer = [0; BUFFER_SIZE];
        let size = stream.read(&mut buffer).ok()?;
        let buffer = &buffer[..size];

        let mut headers = [EMPTY_HEADER; 100];
        let mut request = HttpRequest::new(&mut headers);
        let byte_offset = request.parse(buffer).ok()?;

        let rtype = RequestType::try_from(request.method?).ok()?;
        let path = request.path?.to_owned();
        let path = match path.find('?') {
            Some(pos) => path.chars().into_iter().take(pos).collect(),
            None => path,
        };

        let mut headers = HashMap::new();

        request.headers.to_vec().iter().for_each(|header| {
            headers.insert(
                header.name.to_owned(),
                String::from_utf8_lossy(header.value).to_string(),
            );
        });

        let byte_offset = match byte_offset {
            httparse::Status::Complete(data) => data,
            _ => return None,
        };

        let data = buffer[byte_offset..].to_vec();

        Some(Self {
            rtype,
            path,
            headers,
            data,
            addr,
        })
    }
}
