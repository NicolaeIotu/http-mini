use std::env;
use std::net::{AddrParseError, IpAddr};
use std::num::ParseIntError;

/// # Get port and IP address from command line arguments
///
/// Retrieve command line arguments which can be used as startup
/// port and IP address.
///
/// Defaults:
/// * IP address: "::"
/// * port      : 8080
pub fn get_params() -> (String, i32) {
    let mut address: Option<String> = None;
    let mut port: Option<i32> = None;

    for argument in env::args() {
        if port.is_none() {
            let x_port: Result<i32, ParseIntError> = argument.parse();
            if x_port.is_ok() {
                port = Option::from(x_port.unwrap()); // grcov-excl-line
                continue; // grcov-excl-line
            }
        }
        if address.is_none() {
            let x_addr: Result<IpAddr, AddrParseError> = argument.parse();
            if x_addr.is_ok() {
                address = Option::from(argument); // grcov-excl-line
                continue; // grcov-excl-line
            }
        }
    }

    (address.unwrap_or("::".to_string()), port.unwrap_or(8080))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_params() {
        let (address, port) = get_params();
        assert_eq!(address.as_str(), "::");
        assert_eq!(port, 8080);
    }
}
