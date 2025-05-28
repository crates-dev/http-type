use crate::*;

///  Response body
pub type ResponseBody = Vec<u8>;
///  Response body string
pub type ResponseBodyString = String;
///  Response headers key
pub type ResponseHeadersKey = String;
///  Response headers value
pub type ResponseHeadersValue = String;
///  Response headers
pub type ResponseHeaders = HashMapXxHash3_64<ResponseHeadersKey, ResponseHeadersValue>;
/// Response version
pub type ResponseVersion = String;
/// Response status code
pub type ResponseStatusCode = usize;
/// Response reason phrase
pub type ResponseReasonPhrase = String;
///  Response result
pub type ResponseResult = Result<(), ResponseError>;
/// Response data
pub type ResponseData = Vec<u8>;
/// Response data string
pub type ResponseDataString = String;
/// RwLockReadGuardResponse
pub type RwLockReadGuardResponse<'a> = RwLockReadGuard<'a, Response>;
/// RwLockWriteGuardResponse
pub type RwLockWriteGuardResponse<'a> = RwLockWriteGuard<'a, Response>;
/// OptionResponseHeadersValue
pub type OptionResponseHeadersValue = Option<ResponseHeadersValue>;
