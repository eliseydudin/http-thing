use std::{
    collections::HashMap,
    fmt::{self, Write as _},
};

pub struct Response {
    status: u16,
    status_as_str: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    WriteError(#[from] fmt::Error),
}

impl Response {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        <Self as Default>::default()
    }

    pub(crate) fn build(mut self) -> Result<Vec<u8>, Error> {
        let mut buffer = String::new();
        write!(
            &mut buffer,
            "HTTP/1.1 {} {}\r\n",
            self.status, self.status_as_str
        )?;

        self.headers.iter().for_each(|(header, data)| {
            if let Err(e) = write!(buffer, "{header}: {data}\r\n") {
                log::error!("Cannot write to the buffer! {e}");
            }
        });

        if self.body.is_some() {
            let mut size = 0;
            self.body = self.body.inspect(|v| size = v.len());
            write!(buffer, "content-length: {size}\r\n")?;
        }

        buffer += "\r\n";
        let mut buffer_bytes = buffer.as_bytes().to_vec();

        if let Some(mut body) = self.body {
            buffer_bytes.append(&mut body);
        }

        Ok(buffer_bytes)
    }

    #[inline]
    #[must_use]
    pub fn header<S1, S2>(mut self, header: S1, data: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.headers.insert(header.into(), data.into());
        self
    }

    #[must_use]
    #[inline]
    pub fn status<S: Into<String>>(mut self, status: u16, as_str: S) -> Self {
        self.status = status;
        self.status_as_str = as_str.into();
        self
    }

    #[must_use]
    #[inline]
    pub fn body(mut self, body: &[u8]) -> Self {
        self.body = Some(body.to_owned());
        self
    }
}

impl Default for Response {
    #[inline]
    fn default() -> Self {
        Self {
            status: 200,
            status_as_str: "OK".to_owned(),
            headers: HashMap::new(),
            body: None,
        }
    }
}
