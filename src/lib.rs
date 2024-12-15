mod traits;
mod utils;

use crate::utils::{app, fs};
use std::env;
use std::process::exit;
use utils::http_server;

/// # Start mini-http server
///
/// Example:
///
/// **File main.rs**
/// ```
/// use std::io::Error;
/// use std::net::TcpListener;
/// use std::process::exit;
/// use std::thread;
/// use std::thread::sleep;
/// use std::time::Duration;
///
/// extern crate http_mini_lib;
///
/// fn main() {
///     println!("Starting server...");
///     let t = thread::spawn(move || {
///         http_mini_lib::start();
///     });
///
///     sleep(Duration::new(1, 0));
///     drop(t);
/// }
/// ```
///
/// **shell**:
/// > rustc --extern mini_http=src/external/libmini_http_lib.rlib ./src/main.rs
///
/// > main
// grcov-excl-start
pub fn start() {
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
