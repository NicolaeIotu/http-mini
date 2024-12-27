use crate::errors::missing_source_directory::MissingSourceDirectoryError;

use std::convert::Infallible;
use std::net::{AddrParseError, IpAddr};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::{env, fs};

/// # Get source directory, port and IP address from command line arguments
///
/// Retrieve command line arguments which can be used as source directory,
/// startup port and IP address.
///
/// Source directory is mandatory.
///
/// Defaults:
/// * IP address: "::"
/// * port      : 8080
pub fn get_params() -> Result<(String, i32, PathBuf), MissingSourceDirectoryError> {
    let mut address: Option<String> = None;
    let mut port: Option<i32> = None;
    let mut source_dir: Option<PathBuf> = None;

    let mut first_argument = true;

    for argument in env::args() {
        if first_argument {
            first_argument = false;
            continue;
        }
        if argument.starts_with("-") {
            continue;
        }

        if source_dir.is_none() {
            let x_source_dir: Result<PathBuf, Infallible> = argument.parse();
            if x_source_dir.is_ok() {
                let x_source_dir_pathbuf: PathBuf = x_source_dir.unwrap();
                let canonical = fs::canonicalize(x_source_dir_pathbuf);
                if canonical.is_ok() {
                    source_dir = Option::from(canonical.unwrap()); // grcov-excl-line
                    continue; // grcov-excl-line
                }
            }
        }
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

    // source directory is mandatory when not testing
    if source_dir.is_none() {
        if env::var_os("TEST").is_some() || cfg!(test) {
            source_dir = Option::from(PathBuf::from("./"));
        } else {
            return Err(MissingSourceDirectoryError); // grcov-excl-line
        }
    }

    Ok((
        address.unwrap_or("::".to_string()),
        port.unwrap_or(8080),
        source_dir.unwrap(),
    ))
}
