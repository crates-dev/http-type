#[test]
fn test_enum_to_string() {
    use super::r#enum::*;
    use http_constant::*;
    assert_eq!(
        FileExtension::FileExtension123.to_string(),
        FILE_EXTENSION_123
    )
}

#[test]
fn test_get_content_type() {
    use super::r#enum::*;
    use http_constant::*;
    assert_eq!(
        FileExtension::FileExtension123.get_content_type(),
        APPLICATION_VND_LOTUS_1_2_3
    )
}

#[test]
fn test_parse() {
    use super::r#enum::*;
    use http_constant::*;
    assert_eq!(
        FileExtension::parse(FILE_EXTENSION_123),
        FileExtension::FileExtension123,
    )
}
