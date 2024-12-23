#[derive(Debug)]
pub enum Error {
    Response(String),
    Unknown,
}
