use crate::utils::http_request::{HttpRequest, ParseHttpRequestError};

pub trait StreamTrait {
    fn parse(&self) -> Result<HttpRequest, ParseHttpRequestError>;
}
