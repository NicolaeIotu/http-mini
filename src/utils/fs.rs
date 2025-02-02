use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

/// # Validate path
///
/// Checks:
/// * path is not a symlink
/// * path is not relative
pub fn validate_path(path: &Path) -> bool {
    !path.is_symlink() && !path.is_relative()
}

/// # Retrieve file contents
pub fn get_file_contents(path: &str) -> Result<Vec<u8>, Error> {
    let path_metadata = fs::metadata(path);
    if path_metadata.is_err() {
        return Err(path_metadata.err().unwrap());
    }

    let file_data = fs::read(path);
    if file_data.is_err() {
        return Err(file_data.err().unwrap());
    }

    Ok(file_data.ok().unwrap())
}

const HTML_TEMPLATE: &str =
    "<!DOCTYPE html><html><head><title>###TITLE###</title></head><body>###BODY###</body></html>";
const LISTING_LIST_START: &str = "<ul>";
const LISTING_LIST_END: &str = "</ul>";
const LISTING_LIST_ITEM_START: &str = "<li>";
const LISTING_LIST_ITEM_END: &str = "</li>";
const LISTING_DIR: &str = "<strong>DIR</strong>&nbsp;&nbsp;&nbsp; ";
const LISTING_FILE: &str = "file&nbsp;&nbsp;&nbsp;&nbsp; ";
const LISTING_LINK_PART_1: &str = "<a href=\"";
const LISTING_LINK_PART_2: &str = "\" target=\"_self\">";
const LISTING_LINK_PART_3: &str = "</a>";
const LISTING_DIR_SLASH: &str = "/";

pub fn get_dir_contents_as_html(
    path: &Path,
    source_dir: &Path,
    address: &str,
) -> Result<String, Error> {
    if !path.is_dir() {
        return Err(Error::new(ErrorKind::NotFound, "Not a directory"));
    }

    if !path.starts_with(source_dir) {
        return Err(Error::new(ErrorKind::NotFound, "Access forbidden"));
    }

    let dir_read = fs::read_dir(path);
    if dir_read.is_err() {
        return Err(dir_read.err().unwrap());
    }

    let mut result = format!("{}\n", LISTING_LIST_START);

    let dir_paths = dir_read.unwrap();
    for dir_path in dir_paths {
        if dir_path.is_err() {
            continue;
        }

        let dir_path = dir_path.unwrap();
        let is_dir = dir_path.path().is_dir();
        let entry_display_path = dir_path
            .path()
            .into_os_string()
            .into_string()
            .unwrap()
            .replace(source_dir.to_str().unwrap(), ".");
        let entry_server_uri = format!("{}{}", address, entry_display_path.trim_start_matches("."));

        result = format!(
            "{}{}{}{}{}{}{}{}{}{}\n",
            result,
            LISTING_LIST_ITEM_START,
            if is_dir { LISTING_DIR } else { LISTING_FILE },
            LISTING_LINK_PART_1,
            entry_server_uri,
            LISTING_LINK_PART_2,
            entry_display_path,
            LISTING_LINK_PART_3,
            if is_dir { LISTING_DIR_SLASH } else { "" },
            LISTING_LIST_ITEM_END,
        );
    }
    result = format!("{}\n{}\n", result, LISTING_LIST_END);
    result = HTML_TEMPLATE
        .replace(
            "###TITLE###",
            path.to_str()
                .unwrap()
                .replace(source_dir.to_str().unwrap(), ".")
                .as_str(),
        )
        .replace("###BODY###", result.as_str());

    Ok(result)
}
