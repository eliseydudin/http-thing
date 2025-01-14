use std::collections::HashMap;

pub struct Response {
    status: u16,
    status_as_str: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: 200,
            status_as_str: "OK".to_owned(),
            headers: HashMap::new(),
            body: None,
        }
    }

    pub(crate) fn build(mut self) -> Vec<u8> {
        let mut buffer = String::new();
        buffer += &format!("HTTP/1.1 {} {}\r\n", self.status, self.status_as_str);
        for (header, data) in self.headers {
            buffer += &format!("{header}: {data}\r\n");
        }

        if self.body.is_some() {
            let mut size = 0;
            self.body = self.body.inspect(|v| size = v.len());
            buffer += &format!("content-length: {}\r\n", size);
        }

        buffer += "\r\n";
        let mut buffer = buffer.as_bytes().to_vec();

        if let Some(mut body) = self.body {
            buffer.append(&mut body);
        }

        buffer
    }

    pub fn header(mut self, header: impl Into<String>, data: impl Into<String>) -> Self {
        self.headers.insert(header.into(), data.into());
        self
    }

    pub fn status(mut self, status: u16, as_str: impl Into<String>) -> Self {
        self.status = status;
        self.status_as_str = as_str.into();
        self
    }

    pub fn body(mut self, body: &[u8]) -> Self {
        self.body = Some(body.to_owned());
        self
    }
}
