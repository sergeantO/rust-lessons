use axum::{Router, routing::get};
use tokio::net::TcpListener;

async fn handler() -> &'static str {
    "Hello, world!"
}

fn get_router() -> Router {
    Router::new().route("/hello", get(handler))
}

#[tokio::main]
async fn main() {
    let app = get_router();
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http;
    use http::request::Builder as httpBuilder;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_hello() {
        // given
        let app = get_router();

        // when
        let resp = app
            .oneshot(
                httpBuilder::new()
                    .uri("/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // then
        assert_eq!(resp.status(), 200);

        let body = resp.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, world!")
    }
}
