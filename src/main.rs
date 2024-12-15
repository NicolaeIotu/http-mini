mod traits;
mod utils;

use crate::utils::{app, fs};
use std::env;
use std::process::exit;
use utils::http_server;

/// # Start mini-http server
///
/// A simple single threaded server which can be used during development, or for light traffic applications.
///
/// The application will only serve content which is located in the same directory as own executable.
/// This includes any content in subdirectories.
///
/// By default, the application starts listening on all available interfaces and associated addresses, on port 8080.
///
/// Procedure:
/// * Copy **mini-http** executable to target directory: `cp mini-http /path/to/target/directory/`
/// * Start **mini-http**: `/path/to/target/directory/mini-http`
/// * Open browser i.e. http:///localhost:8080/index.html
///
/// The port and the address can be provided as command line arguments:
/// > mini-http 192.168.1.23 8090
/// >
/// > mini-http 8090 192.168.1.23
///
/// Features:
/// * directory listing
/// * content type detection
// grcov-excl-start
fn main() {
    let binding = fs::get_app_dir().unwrap();
    let app_dir = binding.as_path();
    let (address, port) = app::get_params();

    let executable_path = env::current_exe().unwrap();
    let executable_name = executable_path.file_name().unwrap();
    if executable_name.is_empty() {
        println!("Invalid executable");
        exit(1);
    }

    let result = http_server::run(address.as_str(), port);
    if result.is_err() {
        println!("Http Server Error: {:?}", result.unwrap());
        exit(1);
    }

    let listener = result.unwrap();

    let llv_link_addr: String = format!(
        "http://{}:{}",
        if address == "::" {
            "localhost"
        } else {
            address.as_str()
        },
        port
    );
    let link_addr = llv_link_addr.as_str();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        http_server::handle_connection(stream, app_dir, executable_name, link_addr);
    }
}
// grcov-excl-stop
