mod to_my_http_response;
pub use to_my_http_response::*;
mod to_my_http_full_body_response;
pub use to_my_http_full_body_response::*;
pub type MyHttpResponse =
    http::Response<http_body_util::combinators::BoxBody<bytes::Bytes, String>>;

pub type MyHttpFullBodyResponse = http::Response<http_body_util::Full<bytes::Bytes>>;
