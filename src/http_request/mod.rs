extern crate regex;
use std::net::TcpStream;
use std::io::Read;
use std::collections::HashMap;
use std::str;
use regex::Regex;
use std::error::Error;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub query_params: HashMap<String, String>
}

impl HttpRequest {

    fn parse_key_values(pattern: regex::Regex, match_string: &str) -> HashMap<String, String> {
        
        pattern.captures_iter(&match_string)
            .filter_map(|cap| {
                match (cap.name("key"), cap.name("value")) {
                    (Some(key), Some(value)) => {
                        Some((key.as_str().trim().to_string(), value.as_str().trim().to_string()))
                    },
                    _ => None
                }
            }).collect()
    }

    fn parse_request(raw_request: &str) -> Result<HttpRequest, Box<Error>> {
        let parts: Vec<&str> = raw_request.split("\r\n\r\n").take(2).collect();
        let (headers, body) = (parts[0].to_owned(), parts[1].to_owned());
        let re = Regex::new(r#"^(?P<method>[A-Z]+)\s(https?://)?(?P<path>.+)\sHTTP"#)?;
        let (method, uri) = re.captures(&headers).and_then(|cap| {
            Some((cap.name("method")?.as_str(), cap.name("path")?.as_str()))
        }).unwrap();
        let params = Regex::new(r"(\?.+)")?;
        let header_re = Regex::new(r"(?P<key>.+):\s+(?P<value>.+)\r?\n?")?;
        let param_re = Regex::new(r"(?:\?|&)(?:(?P<key>[^=&]+)=(?P<value>[^=&]+))")?;
        let header_dict = HttpRequest::parse_key_values(header_re, &headers);
        let param_dict = HttpRequest::parse_key_values(param_re, &uri);

        Ok(HttpRequest {
            method: method.to_string(),
            uri: params.replace_all(uri, "").to_string(),
            headers: header_dict,
            body: body.trim().to_string(),
            query_params: param_dict
        })

    }

    pub fn new(stream: &mut TcpStream) -> Result<HttpRequest, Box<Error>> {
        let mut buf = vec![0; 512];
        stream.read(&mut buf)?;
        let raw_request = String::from_utf8(buf)?;
        HttpRequest::parse_request(&raw_request)
    }

    pub fn is_empty_body(&self) -> bool {
        self.body.as_bytes().iter().all(|&x| x == 0)
    }
    
}

#[cfg(test)]
mod tests {
    use super::HttpRequest;
    
    #[test]
    fn test_parse_post_request() {
        let test_post_request = "POST / HTTP/1.1\r\n
                            User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\n
                            Host: www.test.com\r\n
                            Content-Type: application/json\r\n\r\n
                            {'hello': 'world!'}";

        let request = HttpRequest::parse_request(&test_post_request).unwrap();
        assert!(!request.is_empty_body());
        assert_eq!(request.method, "POST".to_string());
        assert_eq!(request.uri, "/".to_string());

        let headers_post_fixture: Vec<(&str, &str)> = vec![
            ("User-Agent", "Mozilla/4.0 (compatible; MSIE5.01; Windows NT)"),
            ("Host", "www.test.com"),
            ("Content-Type", "application/json")
        ];

        headers_post_fixture.iter().for_each(|(k, v)| {
            assert!(
                !match request.headers.get(&k.to_string()) {
                    Some(val) if val == v => val,
                    _ => {
                        println!("{} - {}", k, v);
                        ""
                    }
                }.is_empty()
            );
        });

        assert_eq!(request.body, "{'hello': 'world!'}".to_string());
    }

    #[test]
    fn test_parse_get_request() {
        let test_get_request = "GET /?hello=world&language=rust HTTP/1.1\r\n
                            User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\n
                            Host: www.test.com\r\n
                            Content-Type: application/json\r\n\r\n
                            ";
        let request = HttpRequest::parse_request(&test_get_request).unwrap();
        assert!(request.is_empty_body());
        assert_eq!(request.method, "GET".to_string());
        assert_eq!(request.uri, "/".to_string());

        let headers_get_fixture: Vec<(&str, &str)> = vec![
            ("User-Agent", "Mozilla/4.0 (compatible; MSIE5.01; Windows NT)"),
            ("Host", "www.test.com")
        ];

        headers_get_fixture.iter().for_each(|(k, v)| {
            assert!(
                !match request.headers.get(&k.to_string()) {
                    Some(val) if val == v => val,
                    _ => ""
                    
                }.is_empty()
            );
        });

        let query_params_fixture: Vec<(&str, &str)> = vec![
            ("hello", "world"),
            ("language", "rust")
        ];

        query_params_fixture.iter().for_each(|(k, v)| {
            assert!(
                !match request.query_params.get(&k.to_string()) {
                    Some(val) if val == v => val,
                    _ => ""
                }.is_empty()
            )
        });

        assert!(request.is_empty_body());

    }
}