#[test]
fn test_cookie_builder_new() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.expires, None);
    assert_eq!(cookie.max_age, None);
    assert_eq!(cookie.domain, None);
    assert_eq!(cookie.path, None);
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, true);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, true);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, true);
    assert_eq!(cookie.http_only, true);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, true);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, true);
    assert_eq!(cookie.http_only, true);
    assert_eq!(cookie.same_site, Some("Strict".to_string()));
}

#[test]
fn test_cookie_builder_expires() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.expires("Wed, 21 Oct 2015 07:28:00 GMT");
    assert_eq!(
        cookie.expires,
        Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string())
    );
}

#[test]
fn test_cookie_builder_max_age() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.max_age(3600);
    assert_eq!(cookie.max_age, Some(3600));
}

#[test]
fn test_cookie_builder_domain() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.domain("example.com");
    assert_eq!(cookie.domain, Some("example.com".to_string()));
}

#[test]
fn test_cookie_builder_path() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.path("/admin");
    assert_eq!(cookie.path, Some("/admin".to_string()));
}

#[test]
fn test_cookie_builder_secure() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.secure();
    assert_eq!(cookie.secure, true);
}

#[test]
fn test_cookie_builder_http_only() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.http_only();
    assert_eq!(cookie.http_only, true);
}

#[test]
fn test_cookie_builder_same_site() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.same_site("Strict");
    assert_eq!(cookie.same_site, Some("Strict".to_string()));
}

#[test]
fn test_cookie_builder_chaining() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie
        .expires("Wed, 21 Oct 2015 07:28:00 GMT")
        .max_age(3600)
        .domain("example.com")
        .path("/admin")
        .secure()
        .http_only()
        .same_site("Strict");
    assert_eq!(cookie.name, "session_id");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(
        cookie.expires,
        Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string())
    );
    assert_eq!(cookie.max_age, Some(3600));
    assert_eq!(cookie.domain, Some("example.com".to_string()));
    assert_eq!(cookie.path, Some("/admin".to_string()));
    assert_eq!(cookie.secure, true);
    assert_eq!(cookie.http_only, true);
    assert_eq!(cookie.same_site, Some("Strict".to_string()));
}

#[test]
fn test_cookie_builder_build_basic() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123");
}

#[test]
fn test_cookie_builder_build_empty_name() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::new("", "abc123");
    let result: String = cookie.build();
    assert_eq!(result, "");
}

#[test]
fn test_cookie_builder_build_with_expires() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.expires("Wed, 21 Oct 2015 07:28:00 GMT");
    let result: String = cookie.build();
    assert_eq!(
        result,
        "session_id=abc123; expires=Wed, 21 Oct 2015 07:28:00 GMT"
    );
}

#[test]
fn test_cookie_builder_build_with_max_age() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.max_age(3600);
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; max-age=3600");
}

#[test]
fn test_cookie_builder_build_with_domain() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.domain("example.com");
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; domain=example.com");
}

#[test]
fn test_cookie_builder_build_with_path() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.path("/admin");
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; path=/admin");
}

#[test]
fn test_cookie_builder_build_with_secure() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.secure();
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; secure");
}

#[test]
fn test_cookie_builder_build_with_http_only() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.http_only();
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; httponly");
}

#[test]
fn test_cookie_builder_build_with_same_site() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.same_site("Strict");
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; samesite=Strict");
}

#[test]
fn test_cookie_builder_build_complex() {
    use super::*;
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie
        .expires("Wed, 21 Oct 2015 07:28:00 GMT")
        .max_age(3600)
        .domain("example.com")
        .path("/admin")
        .secure()
        .http_only()
        .same_site("Strict");
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
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
    assert_eq!(cookie.secure, false);
    assert_eq!(cookie.http_only, false);
    assert_eq!(cookie.same_site, None);
}

#[test]
fn test_cookie_builder_build_with_empty_value() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::new("session_id", "");
    let result: String = cookie.build();
    assert_eq!(result, "session_id=");
}

#[test]
fn test_cookie_builder_build_with_special_characters() {
    use super::*;
    let cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123!@#$%^&*()");
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123!@#$%^&*()");
}
