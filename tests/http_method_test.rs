use twelve::net::http::HttpMethod;

#[test]
fn test_http_method_from_bytes_functionality() {
    // Test valid HTTP methods - should return Some(method)
    // using &'static [u8] to handle different byte string lengths
    let valid_cases: Vec<(&[u8], HttpMethod)> = vec![
        (b"GET", HttpMethod::GET),
        (b"POST", HttpMethod::POST),
        (b"PUT", HttpMethod::PUT),
        (b"DELETE", HttpMethod::DELETE),
        (b"PATCH", HttpMethod::PATCH),
        (b"HEAD", HttpMethod::HEAD),
        (b"OPTIONS", HttpMethod::OPTIONS),
        (b"CONNECT", HttpMethod::CONNECT),
        (b"TRACE", HttpMethod::TRACE),
    ];

    for (bytes, expected_method) in valid_cases {
        let result = HttpMethod::from_bytes(bytes);
        assert_eq!(
            result, 
            Some(expected_method),
            "Expected {:?} from bytes {:?}, got {:?}",
            expected_method, 
            String::from_utf8_lossy(bytes),
            result
        );

        // Extended: Verify as_str() consistency for valid methods
        assert_eq!(
            expected_method.as_str().as_bytes(),
            bytes,
            "as_str() mismatch for {:?}",
            expected_method
        );
    }

    // Test invalid HTTP methods - should return None
    let invalid_cases: Vec<&[u8]> = vec![
        b"INVALID",
        b"get",       // lowercase
        b"Get",       // mixed case
        b"",          // empty bytes
        b"GET ",      // with trailing space
        b"GET123",    // with extra characters
        b"HEADER",    // similar but not exact
        b"POSTED",    // similar but not exact
        b"DELETE ",   // with space
        b"\x00",      // null byte
        b"CONNECTED", // too long
        // Extended cases
        b"PUSH",      // non-standard
        b"FETCH",     // non-standard
        b"POST\t",    // with tab
        b"PUT\n",     // with newline
    ];

    for bytes in invalid_cases {
        let result = HttpMethod::from_bytes(bytes);
        assert_eq!(
            result,
            None,
            "Expected None for bytes {:?}, got {:?}",
            String::from_utf8_lossy(bytes),
            result
        );
    }

    // Test edge cases
    let edge_cases: Vec<&[u8]> = vec![
        b"GET\0",     // GET with null terminator
        b"POST\n",    // with newline
        b"PUT\r",     // with carriage return
        b"HEAD\t",    // with tab
        b"OPTIONS ",  // trailing space
        b" CONNECT",  // leading space
        b"TRACE\r\n", // CRLF
    ];

    for bytes in edge_cases {
        let result = HttpMethod::from_bytes(bytes);
        assert_eq!(
            result,
            None,
            "Expected None for edge case bytes {:?}, got {:?}",
            String::from_utf8_lossy(bytes),
            result
        );
    }
}

#[test]
fn test_http_method_as_str_functionality() {
    // Verify as_str returns expected string literals
    assert_eq!(HttpMethod::GET.as_str(), "GET");
    assert_eq!(HttpMethod::POST.as_str(), "POST");
    assert_eq!(HttpMethod::PUT.as_str(), "PUT");
    assert_eq!(HttpMethod::DELETE.as_str(), "DELETE");
    assert_eq!(HttpMethod::PATCH.as_str(), "PATCH");
    assert_eq!(HttpMethod::HEAD.as_str(), "HEAD");
    assert_eq!(HttpMethod::OPTIONS.as_str(), "OPTIONS");
    assert_eq!(HttpMethod::CONNECT.as_str(), "CONNECT");
    assert_eq!(HttpMethod::TRACE.as_str(), "TRACE");
}

#[test]
fn test_http_method_as_str_static_lifetime() {
    // Ensure as_str returns &'static str as implied by signature
    let static_str: &'static str = HttpMethod::DELETE.as_str();
    assert_eq!(static_str, "DELETE");
}

#[test]
fn test_http_method_derived_traits() {
    // Test Debug
    assert_eq!(format!("{:?}", HttpMethod::PUT), "PUT");

    // Test PartialEq
    assert_eq!(HttpMethod::GET, HttpMethod::GET);
    assert_ne!(HttpMethod::GET, HttpMethod::POST);

    // Test Copy/Clone
    let original = HttpMethod::OPTIONS;
    let copied = original;
    let cloned = original.clone();
    assert_eq!(original, copied);
    assert_eq!(original, cloned);
}

#[test]
fn test_http_method_repr_u8_values() {
    // Verify #[repr(u8)] assigns sequential values
    assert_eq!(HttpMethod::GET as u8, 0);
    assert_eq!(HttpMethod::POST as u8, 1);
    assert_eq!(HttpMethod::PUT as u8, 2);
    assert_eq!(HttpMethod::DELETE as u8, 3);
    assert_eq!(HttpMethod::PATCH as u8, 4);
    assert_eq!(HttpMethod::HEAD as u8, 5);
    assert_eq!(HttpMethod::OPTIONS as u8, 6);
    assert_eq!(HttpMethod::CONNECT as u8, 7);
    assert_eq!(HttpMethod::TRACE as u8, 8);
}
