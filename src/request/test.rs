use crate::*;

#[tokio::test]
async fn request_config_from_json() {
    let request_config_json: &'static str = r#"
    {
        "buffer_size": 8192,
        "max_request_line_size": 8192,
        "max_path_size": 8192,
        "max_query_size": 8192,
        "max_header_line_size": 8192,
        "max_header_count": 100,
        "max_header_key_size": 8192,
        "max_header_value_size": 8192,
        "max_body_size": 2097152,
        "max_ws_frame_size": 65536,
        "max_ws_frames_count": 6000,
        "http_read_timeout_ms": 6000,
        "ws_read_timeout_ms": 1800000
    }
    "#;
    let request_config: RequestConfig = RequestConfig::from_json(request_config_json).unwrap();
    let new_request_config: RequestConfig = RequestConfig::new().await;
    new_request_config
        .buffer_size(8192)
        .await
        .max_request_line_size(8192)
        .await
        .max_path_size(8192)
        .await
        .max_query_size(8192)
        .await
        .max_header_line_size(8192)
        .await
        .max_header_count(100)
        .await
        .max_header_key_size(8192)
        .await
        .max_header_value_size(8192)
        .await
        .max_body_size(2097152)
        .await
        .max_ws_frame_size(65536)
        .await
        .max_ws_frames_count(6000)
        .await
        .http_read_timeout_ms(6000)
        .await
        .ws_read_timeout_ms(1800000)
        .await;
    assert_eq!(request_config, new_request_config);
}

#[tokio::test]
async fn request_config_security_levels() {
    let default_config: RequestConfig = RequestConfig::new().await;
    let low_config: RequestConfig = RequestConfig::low_security().await;
    let high_config: RequestConfig = RequestConfig::high_security().await;
    let default_data: RequestConfigData = default_config.get_data().await;
    let low_data: RequestConfigData = low_config.get_data().await;
    let high_data: RequestConfigData = high_config.get_data().await;
    assert!(low_data.get_max_body_size() > default_data.get_max_body_size());
    assert!(high_data.get_max_body_size() < default_data.get_max_body_size());
    assert!(low_data.get_http_read_timeout_ms() > default_data.get_http_read_timeout_ms());
    assert!(high_data.get_http_read_timeout_ms() < default_data.get_http_read_timeout_ms());
}

#[test]
fn request_default() {
    let request: Request = Request::default();
    assert!(request.get_method().is_unknown());
    assert!(request.get_host().is_empty());
    assert!(request.get_path().is_empty());
    assert!(request.get_querys().is_empty());
    assert!(request.get_headers().is_empty());
    assert!(request.get_body().is_empty());
}

#[test]
fn request_query_operations() {
    let mut request: Request = Request::default();
    request
        .querys
        .insert("key1".to_string(), "value1".to_string());
    request
        .querys
        .insert("key2".to_string(), "value2".to_string());
    assert_eq!(request.try_get_query("key1"), Some("value1".to_string()));
    assert_eq!(request.try_get_query("key2"), Some("value2".to_string()));
    assert_eq!(request.try_get_query("key3"), None);
    assert_eq!(request.get_query("key1"), "value1".to_string());
}

#[test]
fn request_header_operations() {
    let mut request: Request = Request::default();
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("value1".to_string());
    values.push_back("value2".to_string());
    request.headers.insert("content-type".to_string(), values);
    assert!(request.has_header("content-type"));
    assert!(!request.has_header("authorization"));
    assert_eq!(request.get_header_size("content-type"), 2);
    assert_eq!(
        request.try_get_header_front("content-type"),
        Some("value1".to_string())
    );
    assert_eq!(
        request.try_get_header_back("content-type"),
        Some("value2".to_string())
    );
    assert_eq!(request.get_headers_size(), 1);
    assert_eq!(request.get_headers_values_size(), 2);
}

#[test]
fn request_has_header_value() {
    let mut request: Request = Request::default();
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("application/json".to_string());
    values.push_back("text/html".to_string());
    request.headers.insert("accept".to_string(), values);
    assert!(request.has_header_value("accept", "application/json"));
    assert!(request.has_header_value("accept", "text/html"));
    assert!(!request.has_header_value("accept", "text/xml"));
    assert!(!request.has_header_value("content-type", "application/json"));
}

#[test]
fn request_body_operations() {
    let request: Request = Request {
        body: b"hello world".to_vec(),
        ..Default::default()
    };
    assert_eq!(request.get_body(), b"hello world");
    assert_eq!(request.get_body_string(), "hello world");
}

#[test]
fn request_body_string_utf8() {
    let body: Vec<u8> = "你好世界".as_bytes().to_vec();
    let request: Request = Request {
        body: body.clone(),
        ..Default::default()
    };
    assert_eq!(request.get_body_string(), "你好世界");
}

#[test]
fn request_body_json() {
    let json: &'static str = r#"{"name":"test","value":123}"#;
    let request: Request = Request {
        body: json.as_bytes().to_vec(),
        ..Default::default()
    };
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }
    let result: TestData = request.try_get_body_json().unwrap();
    assert_eq!(result.name, "test");
    assert_eq!(result.value, 123);
}

#[test]
fn request_method_checks() {
    let request: Request = Request {
        method: Method::Get,
        ..Default::default()
    };
    assert!(request.is_get_method());
    assert!(!request.is_post_method());
    assert!(!request.is_put_method());
    assert!(!request.is_delete_method());
    assert!(!request.is_patch_method());
    assert!(!request.is_head_method());
    assert!(!request.is_options_method());
    assert!(!request.is_connect_method());
    assert!(!request.is_trace_method());
    assert!(!request.is_unknown_method());
}

#[test]
fn request_version_checks() {
    let request: Request = Request {
        version: HttpVersion::Http1_1,
        ..Default::default()
    };
    assert!(request.is_http1_1_version());
    assert!(request.is_http1_1_or_higher_version());
    assert!(request.is_http_version());
    assert!(!request.is_http0_9_version());
    assert!(!request.is_http1_0_version());
    assert!(!request.is_http2_version());
    assert!(!request.is_http3_version());
    assert!(!request.is_unknown_version());
}

#[test]
fn request_upgrade_type_checks() {
    let mut request: Request = Request::default();
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("websocket".to_string());
    request.headers.insert("upgrade".to_string(), values);
    assert!(request.is_ws_upgrade_type());
    assert!(!request.is_h2c_upgrade_type());
    assert!(!request.is_tls_upgrade_type());
    assert!(!request.is_unknown_upgrade_type());
}

#[test]
fn request_keep_alive_checks() {
    let mut request: Request = Request {
        version: HttpVersion::Http1_1,
        ..Default::default()
    };
    assert!(request.is_enable_keep_alive());
    assert!(!request.is_disable_keep_alive());
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("close".to_string());
    request.headers.insert("connection".to_string(), values);
    assert!(!request.is_enable_keep_alive());
    assert!(request.is_disable_keep_alive());
}

#[test]
fn request_error_http_status() {
    let error: RequestError = RequestError::RequestTooLong(HttpStatus::BadRequest);
    assert_eq!(error.get_http_status(), HttpStatus::BadRequest);
    assert_eq!(error.get_http_status_code(), 400);
    let error: RequestError = RequestError::ReadTimeout(HttpStatus::RequestTimeout);
    assert_eq!(error.get_http_status(), HttpStatus::RequestTimeout);
    assert_eq!(error.get_http_status_code(), 408);
}

#[test]
fn request_error_default() {
    let error: RequestError = RequestError::default();
    assert_eq!(error.get_http_status(), HttpStatus::InternalServerError);
}

#[test]
fn request_error_from_io_error() {
    let io_error: std::io::Error = std::io::Error::new(ErrorKind::ConnectionReset, "reset");
    let request_error: RequestError = RequestError::from(io_error);
    assert!(matches!(request_error, RequestError::ClientDisconnected(_)));
    let io_error: std::io::Error = std::io::Error::other("other");
    let request_error: RequestError = RequestError::from(io_error);
    assert!(matches!(request_error, RequestError::ReadConnection(_)));
}

#[tokio::test]
async fn request_error_from_elapsed() {
    let elapsed: Elapsed = tokio::time::timeout(Duration::from_millis(0), async {
        tokio::time::sleep(Duration::from_secs(10)).await;
    })
    .await
    .unwrap_err();
    let request_error: RequestError = RequestError::from(elapsed);
    assert!(matches!(request_error, RequestError::ReadTimeout(_)));
}

#[test]
fn request_error_from_parse_int_error() {
    let parse_error: ParseIntError = "abc".parse::<usize>().unwrap_err();
    let request_error: RequestError = RequestError::from(parse_error);
    assert!(matches!(
        request_error,
        RequestError::InvalidContentLength(_)
    ));
}

#[test]
fn request_config_data_default() {
    let config: RequestConfigData = RequestConfigData::default();
    assert!(config.get_buffer_size() > 0);
    assert!(config.get_max_body_size() > 0);
    assert!(config.get_http_read_timeout_ms() > 0);
}

#[test]
fn request_config_data_security_presets() {
    let low: RequestConfigData = RequestConfigData::low_security();
    let high: RequestConfigData = RequestConfigData::high_security();
    let default: RequestConfigData = RequestConfigData::default();
    assert!(low.get_max_body_size() > default.get_max_body_size());
    assert!(high.get_max_body_size() < default.get_max_body_size());
}
