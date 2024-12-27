mod errors;
mod traits;
pub mod utils;

use crate::utils::app;
use std::env;
use std::process::exit;
use utils::http_server;

/// # Using http-mini library:
///
/// **File main.rs**
/// ```
/// extern crate http_mini_lib;
///
/// fn main() {
///     http_mini_lib::start();
/// }
/// ```
// grcov-excl-start
pub fn start() {
    let get_params_result = app::get_params();
    if get_params_result.is_err() {
        println!("{}", get_params_result.err().unwrap());
        exit(1);
    }
    let (address, port, source_dir) = get_params_result.unwrap();

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
    let source_dir_path = source_dir.as_path();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        http_server::handle_connection(stream, source_dir_path, executable_name, link_addr);
    }
}
// grcov-excl-stop

#[cfg(test)]
mod tests {
    use crate::start;
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_start() {
        let no_panic = true;

        let t = thread::spawn(move || {
            start();
        });

        sleep(Duration::new(1, 0));
        drop(t);

        assert_eq!(no_panic, true);
    }
}
