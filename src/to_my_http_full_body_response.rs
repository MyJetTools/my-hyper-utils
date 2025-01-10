use http::header::*;
use http_body_util::BodyExt;

pub fn compile_full_body<TResult>(
    status_code: http::StatusCode,
    version: http::Version,
    headers: HeaderMap,
    body: Vec<u8>,
    compiler: impl Fn(
        http::response::Builder,
        http_body_util::Full<bytes::Bytes>,
    ) -> http::Response<TResult>,
) -> http::Response<TResult> {
    let mut builder = http::response::Builder::new()
        .status(status_code)
        .version(version);

    let mut has_content_len = false;

    for header in headers {
        if let Some(header_name) = header.0 {
            if header_name
                .as_str()
                .eq_ignore_ascii_case(CONTENT_LENGTH.as_str())
            {
                has_content_len = true;
            }

            if header_name
                .as_str()
                .eq_ignore_ascii_case(TRANSFER_ENCODING.as_str())
            {
                continue;
            }

            builder = builder.header(header_name, header.1);
        }
    }

    if body.len() > 0 {
        if !has_content_len {
            builder = builder.header(CONTENT_LENGTH, body.len());
        }
    }

    let full_body = http_body_util::Full::new(bytes::Bytes::from(body));

    compiler(builder, full_body)
}

pub async fn box_body_to_vec(
    body: http_body_util::combinators::BoxBody<bytes::Bytes, String>,
) -> Result<Vec<u8>, String> {
    let collected = body.collect().await?;
    Ok(collected.to_bytes().into())
}
