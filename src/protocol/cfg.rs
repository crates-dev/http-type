#[test]
fn test_protocol_display() {
    use super::*;
    use http_constant::*;
    assert_eq!(Protocol::Http.to_string(), HTTP_LOWERCASE);
    assert_eq!(Protocol::Https.to_string(), HTTPS_LOWERCASE);
    assert_eq!(Protocol::Unknown("ftp".to_string()).to_string(), "ftp");
}

#[test]
fn test_protocol_from_str() {
    use super::*;
    use http_constant::*;
    assert_eq!(HTTP_LOWERCASE.parse::<Protocol>().unwrap(), Protocol::Http);
    assert_eq!(
        HTTPS_LOWERCASE.parse::<Protocol>().unwrap(),
        Protocol::Https
    );
    assert_eq!(
        "ftp".parse::<Protocol>().unwrap(),
        Protocol::Unknown("ftp".to_string())
    );
    assert_eq!(
        "".parse::<Protocol>().unwrap(),
        Protocol::Unknown("".to_string())
    );
    assert_eq!("HTTP".parse::<Protocol>().unwrap(), Protocol::Http);
    assert_eq!("HTTPS".parse::<Protocol>().unwrap(), Protocol::Https);
}

#[test]
fn test_protocol_default() {
    use super::*;
    assert_eq!(Protocol::default(), Protocol::Unknown(String::new()));
}

#[test]
fn test_protocol_new() {
    use super::*;
    assert_eq!(Protocol::new(), Protocol::default());
    assert_eq!(Protocol::new(), Protocol::default());
}

#[test]
fn test_protocol_is_http() {
    use super::*;
    assert!(Protocol::Http.is_http());
    assert!(!Protocol::Https.is_http());
    assert!(!Protocol::Unknown("http".to_string()).is_http());
    assert!(!Protocol::Unknown("HTTP".to_string()).is_http());
    assert!(!Protocol::Unknown("ftp".to_string()).is_http());
}

#[test]
fn test_protocol_is_https() {
    use super::*;
    assert!(!Protocol::Http.is_https());
    assert!(Protocol::Https.is_https());
    assert!(!Protocol::Unknown("https".to_string()).is_https());
    assert!(!Protocol::Unknown("HTTPS".to_string()).is_https());
    assert!(!Protocol::Unknown("ftp".to_string()).is_https());
}

#[test]
fn test_protocol_get_port() {
    use super::*;
    assert_eq!(Protocol::Http.get_port(), 80);
    assert_eq!(Protocol::Https.get_port(), 443);
    assert_eq!(Protocol::Unknown("ftp".to_string()).get_port(), 80);
    assert_eq!(Protocol::Unknown("custom".to_string()).get_port(), 80);
}

#[test]
fn test_protocol_clone() {
    use super::*;
    let protocol: Protocol = Protocol::Http;
    let cloned_protocol: Protocol = protocol.clone();
    assert_eq!(protocol, cloned_protocol);
    let unknown_protocol: Protocol = Protocol::Unknown("custom".to_string());
    let cloned_unknown: Protocol = unknown_protocol.clone();
    assert_eq!(unknown_protocol, cloned_unknown);
}

#[test]
fn test_protocol_debug() {
    use super::*;
    let protocol: Protocol = Protocol::Http;
    let debug_str: String = format!("{protocol:?}");
    assert_eq!(debug_str, "Http");
    let unknown_protocol: Protocol = Protocol::Unknown("custom".to_string());
    let unknown_debug_str: String = format!("{unknown_protocol:?}");
    assert_eq!(unknown_debug_str, "Unknown(\"custom\")");
}

#[test]
fn test_protocol_equality() {
    use super::*;
    assert_eq!(Protocol::Http, Protocol::Http);
    assert_ne!(Protocol::Http, Protocol::Https);
    assert_eq!(
        Protocol::Unknown("custom".to_string()),
        Protocol::Unknown("custom".to_string())
    );
    assert_ne!(
        Protocol::Unknown("custom1".to_string()),
        Protocol::Unknown("custom2".to_string())
    );
    assert_ne!(Protocol::Http, Protocol::Unknown("http".to_string()));
}

#[test]
fn test_protocol_case_sensitivity() {
    use super::*;
    use http_constant::*;
    assert_eq!(HTTP_LOWERCASE.parse::<Protocol>().unwrap(), Protocol::Http);
    assert_eq!("HTTP".parse::<Protocol>().unwrap(), Protocol::Http);
    assert_eq!(
        HTTPS_LOWERCASE.parse::<Protocol>().unwrap(),
        Protocol::Https
    );
    assert_eq!("HTTPS".parse::<Protocol>().unwrap(), Protocol::Https);
}

#[test]
fn test_protocol_all_variants() {
    use super::*;
    let protocols: Vec<Protocol> = vec![
        Protocol::Http,
        Protocol::Https,
        Protocol::Unknown("ftp".to_string()),
        Protocol::Unknown("custom".to_string()),
    ];
    for protocol in protocols {
        let display_str: String = protocol.to_string();
        assert!(
            !display_str.is_empty() || matches!(protocol, Protocol::Unknown(ref s) if s.is_empty())
        );
        let debug_str: String = format!("{protocol:?}");
        assert!(!debug_str.is_empty());
        let port: u16 = protocol.get_port();
        assert!(port == 80 || port == 443);
    }
}

#[test]
fn test_protocol_unknown_with_empty_string() {
    use super::*;
    let protocol: Protocol = Protocol::Unknown("".to_string());
    assert_eq!(protocol.to_string(), "");
    assert!(!protocol.is_http());
    assert!(!protocol.is_https());
    assert_eq!(protocol.get_port(), 80);
    assert_eq!(format!("{protocol:?}"), "Unknown(\"\")");
}

#[test]
fn test_protocol_unknown_with_special_characters() {
    use super::*;
    let protocol: Protocol = Protocol::Unknown("custom-protocol".to_string());
    assert_eq!(protocol.to_string(), "custom-protocol");
    assert!(!protocol.is_http());
    assert!(!protocol.is_https());
    assert_eq!(protocol.get_port(), 80);
    assert_eq!(format!("{protocol:?}"), "Unknown(\"custom-protocol\")");
}

#[test]
fn test_protocol_pattern_matching() {
    use super::*;
    let protocol: Protocol = Protocol::Http;
    match protocol {
        Protocol::Http => assert!(true),
        Protocol::Https => panic!("Should not match HTTPS"),
        Protocol::Unknown(_) => panic!("Should not match Unknown"),
    }
}

#[test]
fn test_protocol_unknown_pattern_matching() {
    use super::*;
    let protocol: Protocol = Protocol::Unknown("custom".to_string());
    match protocol {
        Protocol::Http => panic!("Should not match HTTP"),
        Protocol::Https => panic!("Should not match HTTPS"),
        Protocol::Unknown(ref custom_protocol) => {
            assert_eq!(custom_protocol, "custom");
        }
    }
}

#[test]
fn test_protocol_secure_check() {
    use super::*;
    assert!(!Protocol::Http.is_https());
    assert!(Protocol::Https.is_https());
    assert!(!Protocol::Unknown("http".to_string()).is_https());
    assert!(!Protocol::Unknown("https".to_string()).is_https());
}

#[test]
fn test_protocol_port_mapping() {
    use super::*;
    let http_port: u16 = Protocol::Http.get_port();
    let https_port: u16 = Protocol::Https.get_port();
    let unknown_port: u16 = Protocol::Unknown("custom".to_string()).get_port();
    assert_eq!(http_port, 80);
    assert_eq!(https_port, 443);
    assert_eq!(unknown_port, 80);
    assert_ne!(http_port, https_port);
}

#[test]
fn test_protocol_ordering() {
    use super::*;
    let mut protocols: Vec<Protocol> = vec![
        Protocol::Unknown("z".to_string()),
        Protocol::Https,
        Protocol::Http,
        Protocol::Unknown("a".to_string()),
    ];
    protocols.sort_by(|a, b| {
        let order_a: u8 = match a {
            Protocol::Http => 0,
            Protocol::Https => 1,
            Protocol::Unknown(_) => 2,
        };
        let order_b: u8 = match b {
            Protocol::Http => 0,
            Protocol::Https => 1,
            Protocol::Unknown(_) => 2,
        };
        order_a
            .cmp(&order_b)
            .then_with(|| a.to_string().cmp(&b.to_string()))
    });
    assert_eq!(protocols[0], Protocol::Http);
    assert_eq!(protocols[1], Protocol::Https);
    assert!(matches!(protocols[2], Protocol::Unknown(_)));
    assert!(matches!(protocols[3], Protocol::Unknown(_)));
}

#[test]
fn test_protocol_memory_size() {
    use super::*;
    use std::mem;
    let size: usize = mem::size_of::<Protocol>();
    assert!(size > 0);
    let http_size: usize = mem::size_of_val(&Protocol::Http);
    let unknown_size: usize = mem::size_of_val(&Protocol::Unknown("test".to_string()));
    assert_eq!(http_size, unknown_size);
}

#[test]
fn test_protocol_from_str_error_type() {
    use super::*;
    let result: Result<Protocol, &'static str> = "invalid".parse();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Protocol::Unknown("invalid".to_string()));
}

#[test]
fn test_protocol_common_protocols() {
    use super::*;
    let common_protocols: Vec<&str> = vec!["ftp", "ssh", "telnet", "smtp", "pop3", "imap"];
    for proto_str in common_protocols {
        let protocol: Protocol = proto_str.parse().unwrap();
        assert_eq!(protocol, Protocol::Unknown(proto_str.to_string()));
        assert_eq!(protocol.to_string(), proto_str);
        assert_eq!(protocol.get_port(), 80);
    }
}
