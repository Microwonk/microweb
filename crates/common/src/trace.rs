use axum::Router;
use axum::http::{Request, Response};
use std::time::Duration;
use tower_http::trace::{DefaultMakeSpan, DefaultOnFailure, OnRequest, OnResponse, TraceLayer};
use tracing::{Level, Span};

#[derive(Clone, Copy)]
struct CustomOnRequest;

impl<B> OnRequest<B> for CustomOnRequest {
    fn on_request(&mut self, req: &Request<B>, span: &Span) {
        let method = req.method().as_str();
        let uri = req.uri().to_string();

        span.record("http.method", method);
        span.record("http.uri", &uri);

        tracing::info!(method, uri, "received request");
    }
}

#[derive(Clone, Copy)]
struct CustomOnResponse;

impl<B> OnResponse<B> for CustomOnResponse {
    fn on_response(self, res: &Response<B>, latency: Duration, span: &Span) {
        let status = res.status().as_u16();
        let level = match res.status() {
            s if s.is_client_error() => tracing::Level::WARN,
            s if s.is_server_error() => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        };

        span.record("http.status", status);
        span.record("latency_ms", latency.as_millis());

        match level {
            tracing::Level::INFO => {
                tracing::info!(status, latency_ms = latency.as_millis(), "sent response")
            }
            tracing::Level::WARN => {
                tracing::warn!(status, latency_ms = latency.as_millis(), "sent response")
            }
            tracing::Level::ERROR => {
                tracing::error!(status, latency_ms = latency.as_millis(), "sent response")
            }
            _ => {}
        }
    }
}

pub trait TraceExt {
    fn with_tracing(self) -> Self;
}

impl<S> TraceExt for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn with_tracing(self) -> Self {
        self.layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(CustomOnRequest)
                .on_response(CustomOnResponse)
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        )
    }
}
