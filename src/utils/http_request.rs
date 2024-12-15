use crate::traits::stream_trait::StreamTrait;
use std::fmt::Display;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

/// # Allowed request methods
const REQUEST_METHODS: [&str; 6] = ["GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS"];

/// # Http request parse errors
#[derive(Debug, PartialEq)]
pub enum ParseHttpRequestError {
    BadLen,
    NoMethod,
    UnknownMethod,
    NoPath,
    NoProtocol,
    UnknownProtocol,
}

impl Display for ParseHttpRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParseHttpRequestError::BadLen => "Invalid request format",
                ParseHttpRequestError::NoMethod => "Missing request method",
                ParseHttpRequestError::UnknownMethod => "Unknown request method",
                ParseHttpRequestError::NoPath => "Missing request path",
                ParseHttpRequestError::NoProtocol => "Missing request protocol",
                ParseHttpRequestError::UnknownProtocol => "Unknown request protocol",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Option<String>,
    pub protocol: Option<String>,
    pub path: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl StreamTrait for TcpStream {
    /// # Stream parser
    fn parse(&self) -> Result<HttpRequest, ParseHttpRequestError> {
        let buf_reader = BufReader::new(self);
        let mut lines_iterator = buf_reader.lines();

        let mut request = HttpRequest {
            method: None,
            protocol: None,
            path: None,
            headers: vec![],
            body: None,
        };

        let mut first_line_vector: Vec<&str>;
        let mut has_first_line = false;
        let mut request_line;
        loop {
            request_line = lines_iterator.next();
            if request_line.is_none() {
                break;
            }

            let line_content = request_line.unwrap().unwrap();
            if line_content.is_empty() {
                break;
            }

            if has_first_line {
                // headers
                let line_content_iterator = line_content.splitn(2, ": ").collect::<Vec<&str>>();
                request.headers.push((
                    line_content_iterator[0].to_string(),
                    line_content_iterator[1].to_string(),
                ));
            } else {
                first_line_vector = line_content.splitn(3, ' ').collect::<Vec<&str>>();
                let init_result = init_request(&mut request, &first_line_vector);
                if init_result.is_err() {
                    return Err(init_result.err().unwrap());
                }

                has_first_line = true;
            }
        }

        Ok(request)
    }
}

/// # Process Http request
fn init_request(
    http_request: &mut HttpRequest,
    first_line_vector: &[&str],
) -> Result<(), ParseHttpRequestError> {
    if first_line_vector.len() != 3 {
        return Err(ParseHttpRequestError::BadLen);
    }

    // Method
    if first_line_vector[0].is_empty() {
        return Err(ParseHttpRequestError::NoMethod);
    }
    if !REQUEST_METHODS.contains(&first_line_vector[0]) {
        return Err(ParseHttpRequestError::UnknownMethod);
    }
    http_request.method = Option::from(first_line_vector[0].to_string());

    // Path
    if !first_line_vector[1].is_empty() {
        http_request.path = Option::from(first_line_vector[1].to_string());
    } else {
        return Err(ParseHttpRequestError::NoPath);
    }

    // Protocol
    if first_line_vector[2].is_empty() {
        return Err(ParseHttpRequestError::NoProtocol);
    }
    if !first_line_vector[2].contains("HTTP") {
        return Err(ParseHttpRequestError::UnknownProtocol);
    }
    http_request.protocol = Option::from(first_line_vector[2].to_string());

    Ok(())
}
