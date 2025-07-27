/// Represents supported HTTP content types.
///
/// This enum defines a set of common content types used in HTTP communication,
/// allowing for easy identification and handling of different data formats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    /// The `application/json` content type.
    ///
    /// Used for sending and receiving JSON data.
    ApplicationJson,
    /// The `application/xml` content type.
    ///
    /// Used for sending and receiving XML data.
    ApplicationXml,
    /// The `text/plain` content type.
    ///
    /// Used for plain text data.
    TextPlain,
    /// The `text/html` content type.
    ///
    /// Used for HTML documents.
    TextHtml,
    /// The `application/x-www-form-urlencoded` content type.
    ///
    /// Used for submitting form data in key-value pairs.
    FormUrlEncoded,
    /// An unknown or unrecognized content type.
    ///
    /// This variant is used when the content type does not match any of the predefined variants.
    Unknown,
}
