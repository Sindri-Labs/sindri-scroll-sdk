use async_compression::tokio::write::ZstdEncoder;
use async_trait::async_trait;
use http::Extensions;
use reqwest::{header::HeaderValue, header::CONTENT_ENCODING, Body, Request, Response};
use reqwest_middleware::{Middleware, Next, Result};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;

const BUFFER_SIZE: usize = 4096;
const MIN_BODY_SIZE: usize = 512;

#[derive(Debug)]
pub struct ZstdRequestCompressionMiddleware;

#[async_trait]
impl Middleware for ZstdRequestCompressionMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        // If the request has a body, compress it using zstd.
        if let Some(bytes) = req
            .body()
            .and_then(|b| b.as_bytes())
            .filter(|b| b.len() >= MIN_BODY_SIZE)
        {
            // Create a new request with the same properties.
            let (method, url, headers, version) = (
                req.method().clone(),
                req.url().clone(),
                req.headers().clone(),
                req.version(),
            );
            let mut new_req = Request::new(method, url);
            *new_req.headers_mut() = headers;
            *new_req.version_mut() = version;

            // Swap out the body with a zstd compressed stream of the original.
            let (writer, reader) = tokio::io::duplex(BUFFER_SIZE);
            let body_arc = Arc::new(bytes.to_vec());
            let body_clone = Arc::clone(&body_arc);
            tokio::spawn(async move {
                let mut encoder = ZstdEncoder::new(writer);
                if let Err(e) = encoder.write_all(&body_clone).await {
                    log::error!("Failed to compress body: {}", e);
                }
                let _ = encoder.shutdown().await;
            });
            new_req
                .body_mut()
                .replace(Body::wrap_stream(ReaderStream::new(reader)));

            // Set the `Content-Encoding: zstd` header.
            new_req
                .headers_mut()
                .insert(CONTENT_ENCODING, HeaderValue::from_static("zstd"));

            // Proceed with the new request.
            return next.run(new_req, extensions).await;
        }

        // If no body needs to be compressed, proceed with the original request.
        next.run(req, extensions).await
    }
}
