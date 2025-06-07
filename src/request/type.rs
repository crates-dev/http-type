use crate::*;

/// Request method
pub type RequestMethod = Method;
/// Request host
pub type RequestHost = String;
/// Request version
pub type RequestVersion = HttpVersion;
/// Request path
pub type RequestPath = String;
/// Request querys key
pub type RequestQuerysKey = String;
/// Request querys value
pub type RequestQuerysValue = String;
/// Request querys
pub type RequestQuerys = HashMapXxHash3_64<RequestQuerysKey, RequestQuerysValue>;
///  Request body
pub type RequestBody = Vec<u8>;
///  Request body string
pub type RequestBodyString = String;
/// Request headers key
pub type RequestHeadersKey = String;
/// Request headers value
pub type RequestHeadersValue = String;
/// Request headers
pub type RequestHeaders = HashMapXxHash3_64<RequestHeadersKey, RequestHeadersValue>;
/// Request reader handle result
pub type RequestReaderHandleResult = Result<Request, RequestError>;
/// RwLockReadGuardRequest
pub type RwLockReadGuardRequest<'a> = RwLockReadGuard<'a, Request>;
/// RwLockWriteGuardRequest
pub type RwLockWriteGuardRequest<'a> = RwLockWriteGuard<'a, Request>;
/// OptionRequestQuerysValue
pub type OptionRequestQuerysValue = Option<RequestQuerysValue>;
/// OptionRequestHeadersValue
pub type OptionRequestHeadersValue = Option<RequestHeadersValue>;
