use crate::traits::stream_trait::StreamTrait;
use crate::utils::fs::get_dir_contents_as_html;
use crate::utils::mimes::get_mime_type;
use crate::utils::{fs, http_response};
use std::ffi::OsStr;
use std::io::Error;
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::path::Path;

/// # Setup and start TcpListener
pub fn run(address: &str, mut port: i32) -> Result<TcpListener, Error> {
    if !(1..=65535).contains(&port) {
        println!("Invalid port number {}. Using port 8080", port);
        port = 8080;
    }

    let ip: IpAddr = address.parse().unwrap();
    let addr = SocketAddr::new(ip, port as u16);
    let result_tcp_listener = TcpListener::bind(addr);

    if result_tcp_listener.is_err() {
        return Err(result_tcp_listener.err().unwrap());
    }

    let listener = result_tcp_listener?;

    Ok(listener)
}

/// # Main connections handler
pub fn handle_connection(
    stream: TcpStream,
    source_dir: &Path,
    executable_name: &OsStr,
    address: &str,
) {
    let http_request = stream.parse();
    if http_request.is_err() {
        http_response::send(
            &stream,
            "HTTP/1.1 400 Bad Request",
            None,
            Option::from(Vec::from(http_request.err().unwrap().to_string())),
        );
        return;
    }

    let request = http_request.unwrap();

    // get file contents
    let request_path = request.path;
    if request_path.is_none() {
        http_response::send(&stream, "HTTP/1.1 400 Bad Request", None, None);
        return;
    }

    let file_path = source_dir.join(request_path.unwrap().trim_start_matches('/'));

    // list directory contents with usable links
    if file_path.is_dir() {
        let dir_contents_as_html = get_dir_contents_as_html(&file_path, source_dir, address);
        if dir_contents_as_html.is_err() {
            http_response::send(&stream, "HTTP/1.1 500 Internal Server Error", None, None);
            return;
        }

        http_response::send(
            &stream,
            "HTTP/1.1 200 OK",
            Option::from(vec![("Content-Type".to_string(), "text/html".to_string())]),
            Option::from(Vec::from(dir_contents_as_html.ok().unwrap())),
        );
        return;
    }

    if !fs::validate_path(file_path.as_ref()) {
        http_response::send(&stream, "HTTP/1.1 404 Not Found", None, None);
        return;
    }

    let file_contents = fs::get_file_contents(file_path.to_str().unwrap());
    if file_contents.is_err() {
        http_response::send(
            &stream,
            "HTTP/1.1 400 Bad Request",
            None,
            Option::from(Vec::from(file_contents.err().unwrap().to_string())),
        );
        return;
    }

    // Extra protection. Prevent calling own executable i.e. http://localhost:8080/mini-http !!!
    if file_path.file_name().is_none() || file_path.file_name().unwrap() == executable_name {
        http_response::send(&stream, "HTTP/1.1 400 Bad Request", None, None);
        return;
    }

    // All OK. Show the file.

    let mut response_headers: Vec<(String, String)> = vec![];
    let mut has_content_type = false;
    if file_path.extension().is_some() {
        let mt = get_mime_type(file_path.extension().unwrap().to_str().unwrap());
        has_content_type = true;
        response_headers.push(("Content-Type".to_string(), mt));
    }
    if !has_content_type {
        response_headers.push(("Content-Type".to_string(), "text/plain".to_string()));
    }

    let status_line = format!("{} 200 OK", request.protocol.unwrap());
    http_response::send(
        &stream,
        status_line.as_str(),
        Option::from(response_headers),
        Option::from(file_contents.unwrap()),
    );
}
