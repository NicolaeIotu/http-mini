/// # Start http-mini server
///
/// A simple HTTP server which can be used during development, or for light traffic applications.
///
/// The application will only serve content in a target directory which must be specified at startup.
///
/// By default, the application starts listening on all available interfaces and associated addresses, on port 8080.
///
/// Arguments can be provided in any order:
/// > http-mini /path/to/target/directory 192.168.1.23 8090
///
/// Features:
/// * directory listing
/// * content type detection
// grcov-excl-start
fn main() {
    http_mini_lib::start();
}
// grcov-excl-stop
