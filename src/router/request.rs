use {
    httparse::{Request as HttpRequest, EMPTY_HEADER},
    std::{
        collections::HashMap,
        io::Read,
        net::{SocketAddr, TcpStream},
    },
};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
#[non_exhaustive]
pub enum RequestType {
    Get,
    Post,
}

impl<'st> TryFrom<&'st str> for RequestType {
    type Error = String;

    #[inline]
    fn try_from(value: &'st str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            _ => Err("Unknown request method".to_owned()),
        }
    }
}

#[non_exhaustive]
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

        let mut headers_new = [EMPTY_HEADER; 100];
        let mut request = HttpRequest::new(&mut headers_new);
        let byte_offset = match request.parse(buffer) {
            Ok(size) => size,
            Err(e) => return Err(format!("{e}")),
        };

        let rtype = RequestType::try_from(request.method.ok_or_else(|| "no method".to_owned())?)?;
        let path = request.path.ok_or("unknown")?.to_owned();
        let path = match path.find('?') {
            Some(pos) => path.chars().take(pos).collect(),
            None => path,
        };

        let fullpath = request.path.ok_or_else(|| "no path".to_owned())?.to_owned();
        let query = (request.path.ok_or_else(|| "no path".to_owned())?.to_owned())
            [fullpath.find('?').unwrap_or(fullpath.len())..]
            .to_string();

        let mut headers = HashMap::new();

        request.headers.to_vec().iter().for_each(|header| {
            headers.insert(
                header.name.to_owned(),
                String::from_utf8_lossy(header.value).to_string(),
            );
        });

        let httparse::Status::Complete(byte_offset_new) = byte_offset else {
            return Err("byte_offset wasn't complete".to_owned());
        };

        let data = buffer.get(byte_offset_new..).unwrap_or_default().to_vec();

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
