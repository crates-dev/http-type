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
        "max_ws_frames": 6000,
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
