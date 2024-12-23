use super::{error::Error, r#type::Response};
use http_constant::*;
use std::{borrow::Cow, collections::HashMap, io::Write, net::TcpStream};

impl<'a> Default for Response<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Response<'a> {
    pub fn new() -> Self {
        Response {
            version: Cow::Borrowed(HTTP_VERSION_1_1),
            status_code: 200,
            reason_phrase: Cow::Borrowed(OK),
            headers: HashMap::new(),
            body: Vec::new(),
            response: Vec::new(),
        }
    }

    pub fn version<S: Into<Cow<'a, str>>>(&mut self, version: S) -> &mut Self {
        self.version = version.into();
        self
    }

    pub fn status_code(&mut self, code: usize) -> &mut Self {
        self.status_code = code;
        self
    }

    pub fn reason_phrase<S: Into<Cow<'a, str>>>(&mut self, phrase: S) -> &mut Self {
        self.reason_phrase = phrase.into();
        self
    }

    pub fn header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn body<B: Into<Vec<u8>>>(&mut self, body: B) -> &mut Self {
        self.body = body.into();
        self
    }

    pub fn build(&mut self) -> Vec<u8> {
        let mut response_str: String = String::new();
        response_str.push_str(&format!(
            "{} {} {}{}",
            self.version, self.status_code, self.reason_phrase, HTTP_BR
        ));
        for (key, value) in &self.headers {
            response_str.push_str(&format!("{}: {}{}", key, value, HTTP_BR));
        }
        response_str.push_str(HTTP_BR);
        let mut response_bytes: Vec<u8> = response_str.into_bytes();
        response_bytes.extend_from_slice(&self.body);
        self.response = response_bytes.clone();
        response_bytes
    }

    pub fn send(&mut self, mut stream: &TcpStream) -> Result<(), Error> {
        if self.response.is_empty() {
            self.response = self.build();
        }
        stream
            .write_all(&self.response)
            .map_err(|err| Error::Response(err.to_string()))
            .and_then(|_| Ok(()))
    }
}
