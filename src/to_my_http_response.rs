use http::StatusCode;
use http_body_util::BodyExt;

use crate::{MyHttpFullBodyResponse, MyHttpResponse};

pub trait ToMyHttpResponse {
    fn to_my_http_response(self) -> MyHttpResponse;
}

impl ToMyHttpResponse for (StatusCode, &'_ str) {
    fn to_my_http_response(self) -> MyHttpResponse {
        let (status_code, body) = self;
        let full_body = http_body_util::Full::new(body.as_bytes().to_vec().into());

        http::response::Response::builder()
            .status(status_code)
            .body(full_body.map_err(|itm| itm.to_string()).boxed())
            .unwrap()
    }
}

impl ToMyHttpResponse for (StatusCode, &'_ [u8]) {
    fn to_my_http_response(self) -> MyHttpResponse {
        let (status_code, body) = self;
        let full_body = http_body_util::Full::new(body.to_vec().into());

        http::response::Response::builder()
            .status(status_code)
            .body(full_body.map_err(|itm| itm.to_string()).boxed())
            .unwrap()
    }
}

impl ToMyHttpResponse for (http::response::Builder, &'_ [u8]) {
    fn to_my_http_response(self) -> MyHttpResponse {
        let (builder, body) = self;
        let full_body = http_body_util::Full::new(body.to_vec().into());

        builder
            .body(full_body.map_err(|itm| itm.to_string()).boxed())
            .unwrap()
    }
}

impl ToMyHttpResponse for (http::response::Builder, Vec<u8>) {
    fn to_my_http_response(self) -> MyHttpResponse {
        let (builder, body) = self;
        let full_body = http_body_util::Full::new(body.into());

        builder
            .body(full_body.map_err(|itm| itm.to_string()).boxed())
            .unwrap()
    }
}

impl ToMyHttpResponse for (http::response::Builder, String) {
    fn to_my_http_response(self) -> MyHttpResponse {
        let (builder, body) = self;
        let full_body = http_body_util::Full::new(body.into());

        builder
            .body(full_body.map_err(|itm| itm.to_string()).boxed())
            .unwrap()
    }
}

impl ToMyHttpResponse for (http::response::Builder, &'_ String) {
    fn to_my_http_response(self) -> MyHttpResponse {
        let (builder, body) = self;
        let full_body = http_body_util::Full::new(body.as_bytes().to_vec().into());

        builder
            .body(full_body.map_err(|itm| itm.to_string()).boxed())
            .unwrap()
    }
}

impl ToMyHttpResponse for (http::response::Builder, &'_ str) {
    fn to_my_http_response(self) -> MyHttpResponse {
        let (builder, body) = self;
        let full_body = http_body_util::Full::new(body.as_bytes().to_vec().into());

        builder
            .body(full_body.map_err(|itm| itm.to_string()).boxed())
            .unwrap()
    }
}

impl ToMyHttpResponse for MyHttpFullBodyResponse {
    fn to_my_http_response(self) -> MyHttpResponse {
        let versions = self.version();
        let status = self.status();
        let (parts, full_body) = self.into_parts();

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
}
