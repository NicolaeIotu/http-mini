use std::io::Write;
use std::net::TcpStream;

const CRLF: &str = "\r\n";

/// # Assembles headers
fn build_headers(headers: Vec<(String, String)>, content_length: usize) -> String {
    let mut result = String::new();
    for (key, value) in headers {
        result = format!("{}{}: {}\n", result, key, value);
    }

    result = format!("{}{}: {}\n", result, "Content-Length", content_length);
    result
}

/// # Send a response
pub fn send(
    mut stream: &TcpStream,
    status_line: &str,
    headers: Option<Vec<(String, String)>>,
    contents_option: Option<String>,
) {
    let contents = contents_option.unwrap_or("".to_string());
    let content_length = contents.len();
    let headers_content = build_headers(headers.unwrap_or_default(), content_length);

    let response = format!("{status_line}{CRLF}{headers_content}{CRLF}{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
