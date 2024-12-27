use std::fmt;

const ERR_MS: &str = "Critical: missing absolute path to HTTP server source directory";

pub struct MissingSourceDirectoryError;

impl fmt::Display for MissingSourceDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ERR_MS)
    }
}

impl fmt::Debug for MissingSourceDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ message: {}, file: {}, line: {} }}",
            ERR_MS,
            file!(),
            line!()
        )
    }
}
