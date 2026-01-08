#[test]
fn test_cookie_builder_new() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::new("session_id".to_string(), "abc123".to_string());
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_default() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::default();
    assert_eq!(cookie.name, "");
    assert_eq!(cookie.value, "");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_basic() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_with_expires() {
    use super::*;
    let cookie: CookieBuilder =
        CookieBuilder::parse("session_id=abc123; expires=Wed, 21 Oct 2015 07:28:00 GMT");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(
        cookie.expires,
        Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string())
    );
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_with_max_age() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; max-age=3600");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, Some(3600));
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_with_domain() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; domain=example.com");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, Some("example.com".to_string()));
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_with_path() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; path=/admin");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, Some("/admin".to_string()));
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_with_secure() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; secure");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_with_httponly() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; httponly");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_with_samesite() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; samesite=Strict");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, Some("Strict".to_string()));
}

#[test]
fn test_cookie_builder_parse_complex() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse(
        "session_id=abc123; expires=Wed, 21 Oct 2015 07:28:00 GMT; max-age=3600; domain=example.com; path=/admin; secure; httponly; samesite=Strict",
    );
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(
        cookie.expires,
        Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string())
    );
    assert_eq!(cookie.max_age, Some(3600));
    assert_eq!(cookie.domain, Some("example.com".to_string()));
    assert_eq!(cookie.path, Some("/admin".to_string()));
    assert!(cookie.secure);
    assert!(cookie.http_only);
    assert_eq!(cookie.same_site, Some("Strict".to_string()));
}

#[test]
fn test_cookie_builder_parse_empty_string() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("");
    assert_eq!(cookie.name, "");
    assert_eq!(cookie.value, "");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_name_only() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_with_spaces() {
    use super::*;
    let cookie: CookieBuilder =
        CookieBuilder::parse("  session_id  =  abc123  ;  domain  =  example.com  ;  secure  ");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, Some("example.com".to_string()));
    assert_eq!(cookie.path, None);
    assert!(cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_invalid_max_age() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; max-age=invalid");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_case_insensitive() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse(
        "session_id=abc123; DOMAIN=example.com; SECURE; HTTPONLY; SAMESITE=Strict",
    );
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, Some("example.com".to_string()));
    assert_eq!(cookie.path, None);
    assert!(cookie.secure);
    assert!(cookie.http_only);
    assert_eq!(cookie.same_site, Some("Strict".to_string()));
}

#[test]
fn test_cookie_builder_expires() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("".to_string(), "".to_string());
    cookie.set_expires(Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string()));
    assert_eq!(
        cookie.expires,
        Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string())
    );
}

#[test]
fn test_cookie_builder_max_age() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("".to_string(), "".to_string());
    cookie.set_max_age(Some(3600));
    assert_eq!(cookie.max_age, Some(3600));
}

#[test]
fn test_cookie_builder_domain() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("".to_string(), "".to_string());
    cookie.set_domain(Some("example.com".to_string()));
    assert_eq!(cookie.domain, Some("example.com".to_string()));
}

#[test]
fn test_cookie_builder_path() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("".to_string(), "".to_string());
    cookie.set_path(Some("/admin".to_string()));
    assert_eq!(cookie.path, Some("/admin".to_string()));
}

#[test]
fn test_cookie_builder_secure() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("".to_string(), "".to_string());
    cookie.set_secure(true);
    assert!(cookie.secure);
}

#[test]
fn test_cookie_builder_http_only() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("".to_string(), "".to_string());
    cookie.set_http_only(true);
    assert!(cookie.http_only);
}

#[test]
fn test_cookie_builder_same_site() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("".to_string(), "".to_string());
    cookie.set_same_site(Some("Strict".to_string()));
    assert_eq!(cookie.same_site, Some("Strict".to_string()));
}

#[test]
fn test_cookie_builder_chaining() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id".to_string(), "".to_string());
    cookie
        .set_expires(Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string()))
        .set_max_age(Some(3600))
        .set_domain(Some("example.com".to_string()))
        .set_path(Some("/admin".to_string()))
        .set_secure(true)
        .set_http_only(true)
        .set_same_site(Some("Strict".to_string()));
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "");
    assert_eq!(
        cookie.expires,
        Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string())
    );
    assert_eq!(cookie.max_age, Some(3600));
    assert_eq!(cookie.domain, Some("example.com".to_string()));
    assert_eq!(cookie.path, Some("/admin".to_string()));
    assert!(cookie.secure);
    assert!(cookie.http_only);
    assert_eq!(cookie.same_site, Some("Strict".to_string()));
}

#[test]
fn test_cookie_builder_build_basic() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::new("session_id".to_string(), "abc123".to_string());
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123");
}

#[test]
fn test_cookie_builder_build_empty_name() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::default();
    let result: String = cookie.build();
    assert_eq!(result, "");
}

#[test]
fn test_cookie_builder_build_with_expires() {
    use super::*;
    let mut cookie: CookieBuilder =
        CookieBuilder::new("session_id".to_string(), "abc123".to_string());
    cookie.set_expires(Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string()));
    let result: String = cookie.build();
    assert_eq!(
        result,
        "session_id=abc123; expires=Wed, 21 Oct 2015 07:28:00 GMT"
    );
}

#[test]
fn test_cookie_builder_build_with_max_age() {
    use super::*;
    let mut cookie: CookieBuilder =
        CookieBuilder::new("session_id".to_string(), "abc123".to_string());
    cookie.set_max_age(Some(3600));
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; max-age=3600");
}

#[test]
fn test_cookie_builder_build_with_domain() {
    use super::*;
    let mut cookie: CookieBuilder =
        CookieBuilder::new("session_id".to_string(), "abc123".to_string());
    cookie.set_domain(Some("example.com".to_string()));
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; domain=example.com");
}

#[test]
fn test_cookie_builder_build_with_path() {
    use super::*;
    let mut cookie: CookieBuilder =
        CookieBuilder::new("session_id".to_string(), "abc123".to_string());
    cookie.set_path(Some("/admin".to_string()));
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; path=/admin");
}

#[test]
fn test_cookie_builder_build_with_secure() {
    use super::*;
    let mut cookie: CookieBuilder =
        CookieBuilder::new("session_id".to_string(), "abc123".to_string());
    cookie.set_secure(true);
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; secure");
}

#[test]
fn test_cookie_builder_build_with_http_only() {
    use super::*;
    let mut cookie: CookieBuilder =
        CookieBuilder::new("session_id".to_string(), "abc123".to_string());
    cookie.set_http_only(true);
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; httponly");
}

#[test]
fn test_cookie_builder_build_with_same_site() {
    use super::*;
    let mut cookie: CookieBuilder =
        CookieBuilder::new("session_id".to_string(), "abc123".to_string());
    cookie.set_same_site(Some("Strict".to_string()));
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; samesite=Strict");
}

#[test]
fn test_cookie_builder_build_complex() {
    use super::*;
    let mut cookie: CookieBuilder =
        CookieBuilder::new("session_id".to_string(), "abc123".to_string());
    cookie
        .set_expires(Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string()))
        .set_max_age(Some(3600))
        .set_domain(Some("example.com".to_string()))
        .set_path(Some("/admin".to_string()))
        .set_secure(true)
        .set_http_only(true)
        .set_same_site(Some("Strict".to_string()));
    let result: String = cookie.build();
    assert_eq!(
        result,
        "session_id=abc123; expires=Wed, 21 Oct 2015 07:28:00 GMT; max-age=3600; domain=example.com; path=/admin; secure; httponly; samesite=Strict"
    );
}

#[test]
fn test_cookie_builder_parse_with_semicolon_only() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse(";");
    assert_eq!(cookie.name, "");
    assert_eq!(cookie.value, "");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_with_multiple_semicolons() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123;;;domain=example.com;;");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, Some("example.com".to_string()));
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_with_unknown_attributes() {
    use super::*;
    let cookie: CookieBuilder =
        CookieBuilder::parse("session_id=abc123; unknown=value; anotherflag; domain=example.com");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, Some("example.com".to_string()));
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_negative_max_age() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; max-age=-1");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, Some(-1));
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_zero_max_age() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; max-age=0");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, Some(0));
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_large_max_age() {
    use super::*;
    let cookie: CookieBuilder =
        CookieBuilder::parse("session_id=abc123; max-age=9223372036854775807");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, Some(9223372036854775807));
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_empty_value() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_empty_attribute_value() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; domain=; path=");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, Some("".to_string()));
    assert_eq!(cookie.path, Some("".to_string()));
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_parse_special_characters() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::parse(
        "session_id=abc123!@#$%^&*(); domain=sub.example.com; path=/admin/users",
    );
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123!@#$%^&*()");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, Some("sub.example.com".to_string()));
    assert_eq!(cookie.path, Some("/admin/users".to_string()));
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_parse_empty() {
    use super::*;
    let cookies: Cookies = Cookie::parse("");
    assert!(cookies.is_empty());
}

#[test]
fn test_cookie_parse_single() {
    use super::*;
    let cookies: Cookies = Cookie::parse("session_id=abc123");
    assert_eq!(cookies.len(), 1);
    assert_eq!(cookies.get("session_id"), Some(&"abc123".to_string()));
}

#[test]
fn test_cookie_parse_multiple() {
    use super::*;
    let cookies: Cookies = Cookie::parse("session_id=abc123; user_id=456; theme=dark");
    assert_eq!(cookies.len(), 3);
    assert_eq!(cookies.get("session_id"), Some(&"abc123".to_string()));
    assert_eq!(cookies.get("user_id"), Some(&"456".to_string()));
    assert_eq!(cookies.get("theme"), Some(&"dark".to_string()));
}

#[test]
fn test_cookie_parse_with_spaces() {
    use super::*;
    let cookies: Cookies = Cookie::parse("  session_id  =  abc123  ;  user_id  =  456  ");
    assert_eq!(cookies.len(), 2);
    assert_eq!(cookies.get("session_id"), Some(&"abc123".to_string()));
    assert_eq!(cookies.get("user_id"), Some(&"456".to_string()));
}

#[test]
fn test_cookie_parse_empty_value() {
    use super::*;
    let cookies: Cookies = Cookie::parse("session_id=; user_id=456");
    assert_eq!(cookies.len(), 2);
    assert_eq!(cookies.get("session_id"), Some(&"".to_string()));
    assert_eq!(cookies.get("user_id"), Some(&"456".to_string()));
}

#[test]
fn test_cookie_parse_no_value() {
    use super::*;
    let cookies: Cookies = Cookie::parse("session_id; user_id=456");
    assert_eq!(cookies.len(), 2);
    assert_eq!(cookies.get("session_id"), Some(&"".to_string()));
    assert_eq!(cookies.get("user_id"), Some(&"456".to_string()));
}

#[test]
fn test_cookie_parse_multiple_semicolons() {
    use super::*;
    let cookies: Cookies = Cookie::parse("session_id=abc123;;;user_id=456;;");
    assert_eq!(cookies.len(), 2);
    assert_eq!(cookies.get("session_id"), Some(&"abc123".to_string()));
    assert_eq!(cookies.get("user_id"), Some(&"456".to_string()));
}

#[test]
fn test_cookie_parse_special_characters() {
    use super::*;
    let cookies: Cookies = Cookie::parse("session_id=abc123!@#$%^&*(); user_id=456");
    assert_eq!(cookies.len(), 2);
    assert_eq!(
        cookies.get("session_id"),
        Some(&"abc123!@#$%^&*()".to_string())
    );
    assert_eq!(cookies.get("user_id"), Some(&"456".to_string()));
}

#[test]
fn test_cookie_builder_build_with_empty_value() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::new("session_id".to_string(), "".to_string());
    let result: String = cookie.build();
    assert_eq!(result, "session_id=");
}

#[test]
fn test_cookie_builder_build_with_special_characters() {
    use super::*;
    let cookie: CookieBuilder =
        CookieBuilder::new("session_id".to_string(), "abc123!@#$%^&*()".to_string());
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123!@#$%^&*()");
}
