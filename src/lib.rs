use http::StatusCode;
use http_body_util::BodyExt;

pub type HttpResponse = http::Response<http_body_util::combinators::BoxBody<bytes::Bytes, String>>;

pub type HttpFullBodyResponse = http::Response<http_body_util::Full<bytes::Bytes>>;

pub fn build_content_http_response(status: StatusCode, content: Vec<u8>) -> HttpResponse {
    let full_body = http_body_util::Full::new(bytes::Bytes::from(content));
    let builder = http::Response::builder().status(status);

    builder
        .body(full_body.map_err(|itm| itm.to_string()).boxed())
        .unwrap()
}

pub fn from_full_body(full_body: HttpFullBodyResponse) -> HttpResponse {
    let versions = full_body.version();
    let status = full_body.status();
    let (parts, full_body) = full_body.into_parts();

    let mut builder = http::response::Response::builder()
        .status(status)
        .version(versions);

    for header in parts.headers {
        if let Some(header_name) = header.0 {
            builder = builder.header(header_name, header.1);
        }
    }

    builder
        .body(full_body.map_err(|itm| itm.to_string()).boxed())
        .unwrap()
}

pub fn merge_http_body(builder: http::response::Builder, body: Vec<u8>) -> HttpResponse {
    let full_body = http_body_util::Full::new(bytes::Bytes::from(body));

    builder
        .body(full_body.map_err(|itm| itm.to_string()).boxed())
        .unwrap()
}
