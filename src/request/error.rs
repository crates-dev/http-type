#[derive(Debug)]
pub enum Error {
    HttpReadError(String),
    InvalidHttpRequest(String),
    Unknown,
}
