/// Represents supported HTTP content types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    /// `application/json`
    ApplicationJson,
    /// `application/xml`
    ApplicationXml,
    /// `text/plain`
    TextPlain,
    /// `text/html`
    TextHtml,
    /// `application/x-www-form-urlencoded`
    FormUrlEncoded,
    /// Unknown or unrecognized content type
    Unknown,
}
