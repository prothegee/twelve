use twelve::net::http::HttpMethod;

#[test]
fn test_http_method_from_bytes_functionality() {
    // Test valid HTTP methods - should return Some(method)
    // Using &'static [u8] to handle different byte string lengths
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
