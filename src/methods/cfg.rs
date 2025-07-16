#[test]
fn test_method_display() {
    use super::*;
    use http_constant::*;
    assert_eq!(Method::GET.to_string(), GET);
    assert_eq!(Method::POST.to_string(), POST);
    assert_eq!(Method::PUT.to_string(), PUT);
    assert_eq!(Method::DELETE.to_string(), DELETE);
    assert_eq!(Method::PATCH.to_string(), PATCH);
    assert_eq!(Method::HEAD.to_string(), HEAD);
    assert_eq!(Method::OPTIONS.to_string(), OPTIONS);
    assert_eq!(Method::CONNECT.to_string(), CONNECT);
    assert_eq!(Method::TRACE.to_string(), TRACE);
    assert_eq!(Method::UNKNOWN("CUSTOM".to_string()).to_string(), "CUSTOM");
}

#[test]
fn test_method_from_str() {
    use super::*;
    use http_constant::*;
    assert_eq!(GET.parse::<Method>().unwrap(), Method::GET);
    assert_eq!(POST.parse::<Method>().unwrap(), Method::POST);
    assert_eq!(PUT.parse::<Method>().unwrap(), Method::PUT);
    assert_eq!(DELETE.parse::<Method>().unwrap(), Method::DELETE);
    assert_eq!(PATCH.parse::<Method>().unwrap(), Method::PATCH);
    assert_eq!(HEAD.parse::<Method>().unwrap(), Method::HEAD);
    assert_eq!(OPTIONS.parse::<Method>().unwrap(), Method::OPTIONS);
    assert_eq!(CONNECT.parse::<Method>().unwrap(), Method::CONNECT);
    assert_eq!(TRACE.parse::<Method>().unwrap(), Method::TRACE);
    assert_eq!(
        "CUSTOM".parse::<Method>().unwrap(),
        Method::UNKNOWN("CUSTOM".to_string())
    );
    assert_eq!(
        "".parse::<Method>().unwrap(),
        Method::UNKNOWN("".to_string())
    );
}

#[test]
fn test_method_default() {
    use super::*;
    assert_eq!(Method::default(), Method::UNKNOWN(String::new()));
}

#[test]
fn test_method_new() {
    use super::*;
    assert_eq!(Method::new(), Method::default());
}

#[test]
fn test_method_is_get() {
    use super::*;
    assert!(Method::GET.is_get());
    assert!(!Method::POST.is_get());
    assert!(!Method::PUT.is_get());
    assert!(!Method::DELETE.is_get());
    assert!(!Method::PATCH.is_get());
    assert!(!Method::HEAD.is_get());
    assert!(!Method::OPTIONS.is_get());
    assert!(!Method::CONNECT.is_get());
    assert!(!Method::TRACE.is_get());
    assert!(!Method::UNKNOWN("GET".to_string()).is_get());
}

#[test]
fn test_method_is_post() {
    use super::*;
    assert!(!Method::GET.is_post());
    assert!(Method::POST.is_post());
    assert!(!Method::PUT.is_post());
    assert!(!Method::DELETE.is_post());
    assert!(!Method::PATCH.is_post());
    assert!(!Method::HEAD.is_post());
    assert!(!Method::OPTIONS.is_post());
    assert!(!Method::CONNECT.is_post());
    assert!(!Method::TRACE.is_post());
    assert!(!Method::UNKNOWN("POST".to_string()).is_post());
}

#[test]
fn test_method_is_put() {
    use super::*;
    assert!(!Method::GET.is_put());
    assert!(!Method::POST.is_put());
    assert!(Method::PUT.is_put());
    assert!(!Method::DELETE.is_put());
    assert!(!Method::PATCH.is_put());
    assert!(!Method::HEAD.is_put());
    assert!(!Method::OPTIONS.is_put());
    assert!(!Method::CONNECT.is_put());
    assert!(!Method::TRACE.is_put());
    assert!(!Method::UNKNOWN("PUT".to_string()).is_put());
}

#[test]
fn test_method_is_delete() {
    use super::*;
    assert!(!Method::GET.is_delete());
    assert!(!Method::POST.is_delete());
    assert!(!Method::PUT.is_delete());
    assert!(Method::DELETE.is_delete());
    assert!(!Method::PATCH.is_delete());
    assert!(!Method::HEAD.is_delete());
    assert!(!Method::OPTIONS.is_delete());
    assert!(!Method::CONNECT.is_delete());
    assert!(!Method::TRACE.is_delete());
    assert!(!Method::UNKNOWN("DELETE".to_string()).is_delete());
}

#[test]
fn test_method_is_patch() {
    use super::*;
    assert!(!Method::GET.is_patch());
    assert!(!Method::POST.is_patch());
    assert!(!Method::PUT.is_patch());
    assert!(!Method::DELETE.is_patch());
    assert!(Method::PATCH.is_patch());
    assert!(!Method::HEAD.is_patch());
    assert!(!Method::OPTIONS.is_patch());
    assert!(!Method::CONNECT.is_patch());
    assert!(!Method::TRACE.is_patch());
    assert!(!Method::UNKNOWN("PATCH".to_string()).is_patch());
}

#[test]
fn test_method_is_head() {
    use super::*;
    assert!(!Method::GET.is_head());
    assert!(!Method::POST.is_head());
    assert!(!Method::PUT.is_head());
    assert!(!Method::DELETE.is_head());
    assert!(!Method::PATCH.is_head());
    assert!(Method::HEAD.is_head());
    assert!(!Method::OPTIONS.is_head());
    assert!(!Method::CONNECT.is_head());
    assert!(!Method::TRACE.is_head());
    assert!(!Method::UNKNOWN("HEAD".to_string()).is_head());
}

#[test]
fn test_method_is_options() {
    use super::*;
    assert!(!Method::GET.is_options());
    assert!(!Method::POST.is_options());
    assert!(!Method::PUT.is_options());
    assert!(!Method::DELETE.is_options());
    assert!(!Method::PATCH.is_options());
    assert!(!Method::HEAD.is_options());
    assert!(Method::OPTIONS.is_options());
    assert!(!Method::CONNECT.is_options());
    assert!(!Method::TRACE.is_options());
    assert!(!Method::UNKNOWN("OPTIONS".to_string()).is_options());
}

#[test]
fn test_method_is_connect() {
    use super::*;
    assert!(!Method::GET.is_connect());
    assert!(!Method::POST.is_connect());
    assert!(!Method::PUT.is_connect());
    assert!(!Method::DELETE.is_connect());
    assert!(!Method::PATCH.is_connect());
    assert!(!Method::HEAD.is_connect());
    assert!(!Method::OPTIONS.is_connect());
    assert!(Method::CONNECT.is_connect());
    assert!(!Method::TRACE.is_connect());
    assert!(!Method::UNKNOWN("CONNECT".to_string()).is_connect());
}

#[test]
fn test_method_is_trace() {
    use super::*;
    assert!(!Method::GET.is_trace());
    assert!(!Method::POST.is_trace());
    assert!(!Method::PUT.is_trace());
    assert!(!Method::DELETE.is_trace());
    assert!(!Method::PATCH.is_trace());
    assert!(!Method::HEAD.is_trace());
    assert!(!Method::OPTIONS.is_trace());
    assert!(!Method::CONNECT.is_trace());
    assert!(Method::TRACE.is_trace());
    assert!(!Method::UNKNOWN("TRACE".to_string()).is_trace());
}

#[test]
fn test_method_is_unknown() {
    use super::*;
    assert!(!Method::GET.is_unknown());
    assert!(!Method::POST.is_unknown());
    assert!(!Method::PUT.is_unknown());
    assert!(!Method::DELETE.is_unknown());
    assert!(!Method::PATCH.is_unknown());
    assert!(!Method::HEAD.is_unknown());
    assert!(!Method::OPTIONS.is_unknown());
    assert!(!Method::CONNECT.is_unknown());
    assert!(!Method::TRACE.is_unknown());
    assert!(Method::UNKNOWN("CUSTOM".to_string()).is_unknown());
    assert!(Method::UNKNOWN("".to_string()).is_unknown());
}

#[test]
fn test_method_clone() {
    use super::*;
    let method: Method = Method::GET;
    let cloned_method: Method = method.clone();
    assert_eq!(method, cloned_method);
    let unknown_method: Method = Method::UNKNOWN("CUSTOM".to_string());
    let cloned_unknown: Method = unknown_method.clone();
    assert_eq!(unknown_method, cloned_unknown);
}

#[test]
fn test_method_debug() {
    use super::*;
    let method: Method = Method::GET;
    let debug_str: String = format!("{:?}", method);
    assert_eq!(debug_str, "GET");
    let unknown_method: Method = Method::UNKNOWN("CUSTOM".to_string());
    let unknown_debug_str: String = format!("{:?}", unknown_method);
    assert_eq!(unknown_debug_str, "UNKNOWN(\"CUSTOM\")");
}

#[test]
fn test_method_equality() {
    use super::*;
    assert_eq!(Method::GET, Method::GET);
    assert_ne!(Method::GET, Method::POST);
    assert_eq!(
        Method::UNKNOWN("CUSTOM".to_string()),
        Method::UNKNOWN("CUSTOM".to_string())
    );
    assert_ne!(
        Method::UNKNOWN("CUSTOM1".to_string()),
        Method::UNKNOWN("CUSTOM2".to_string())
    );
    assert_ne!(Method::GET, Method::UNKNOWN("GET".to_string()));
}

#[test]
fn test_method_case_sensitivity() {
    use super::*;
    assert_eq!(
        "get".parse::<Method>().unwrap(),
        Method::UNKNOWN("get".to_string())
    );
    assert_eq!(
        "Get".parse::<Method>().unwrap(),
        Method::UNKNOWN("Get".to_string())
    );
    assert_eq!("POST".parse::<Method>().unwrap(), Method::POST);
    assert_eq!(
        "post".parse::<Method>().unwrap(),
        Method::UNKNOWN("post".to_string())
    );
}

#[test]
fn test_method_all_variants() {
    use super::*;
    let methods: Vec<Method> = vec![
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::PATCH,
        Method::HEAD,
        Method::OPTIONS,
        Method::CONNECT,
        Method::TRACE,
        Method::UNKNOWN("CUSTOM".to_string()),
    ];
    for method in methods {
        let display_str: String = method.to_string();
        assert!(!display_str.is_empty());
        let debug_str: String = format!("{:?}", method);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_method_unknown_with_empty_string() {
    use super::*;
    let method: Method = Method::UNKNOWN("".to_string());
    assert_eq!(method.to_string(), "");
    assert!(method.is_unknown());
    assert_eq!(format!("{:?}", method), "UNKNOWN(\"\")");
}

#[test]
fn test_method_unknown_with_special_characters() {
    use super::*;
    let method: Method = Method::UNKNOWN("CUSTOM-METHOD".to_string());
    assert_eq!(method.to_string(), "CUSTOM-METHOD");
    assert!(method.is_unknown());
    assert_eq!(format!("{:?}", method), "UNKNOWN(\"CUSTOM-METHOD\")");
}
