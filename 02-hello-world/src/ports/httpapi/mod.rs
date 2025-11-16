use std::sync::Arc;

use axum::{Router, extract::State, routing::get};
use tokio::net::TcpListener;

use crate::{app::query::get_hello_world::Repository, di::Container};

pub struct Server<R>
where
    R: Repository + Send + Sync + 'static,
{
    port: u16,
    container: Arc<Container<R>>,
}
impl<R> Server<R>
where
    R: Repository + Send + Sync + 'static,
{
    pub fn new(port: u16, container: Arc<Container<R>>) -> Self {
        Self { port, container }
    }

    pub async fn run(self) {
        let app = get_router(self.container.clone());
        let url = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(url).await.unwrap();

        axum::serve(listener, app).await.unwrap();
    }
}

async fn handler<R>(State(container): State<Arc<Container<R>>>) -> &'static str
where
    R: Repository + Send + Sync + 'static,
{
    container.hello_world_query.execute().await
}

pub fn get_router<R>(container: Arc<Container<R>>) -> Router
where
    R: Repository + Send + Sync + 'static,
{
    Router::new()
        .route("/hello", get(handler))
        .with_state(container)
}

#[cfg(test)]
mod tests {
    use crate::app::query::get_hello_world::InMemoryRepo;

    use super::*;
    use axum::body::Body;
    use axum::http;
    use http::request::Builder as httpBuilder;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    fn setup() -> Arc<Container<InMemoryRepo>> {
        let repo = InMemoryRepo;
        let container = Arc::new(Container::new(repo));
        return container;
    }

    #[tokio::test]
    async fn test_hello() {
        // given
        let container = setup();
        let app = get_router(container);

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

    #[tokio::test]
    async fn test_not_found() {
        // given
        let container = setup();
        let app = get_router(container);

        // when
        let resp = app
            .oneshot(
                httpBuilder::new()
                    .uri("/not_found")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), 404);
    }
}
