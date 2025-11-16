use std::sync::Arc;

use crate::ports::httpimpl::get_router::get_router;
use crate::{
    app::{
        command::create_short_url::CreateShortUrlRepository,
        query::get_full_url::GetFullUrlRepository,
    },
    di::Container,
    id_provider::IDProvider,
};
use tokio::net::TcpListener;

/// сервер приложения
pub struct Server<I, R, Q>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    port: u16,
    container: Arc<Container<I, R, Q>>,
}

impl<I, R, Q> Server<I, R, Q>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    pub fn new(port: u16, container: Arc<Container<I, R, Q>>) -> Self {
        Server { port, container }
    }

    /// Запуск сервера
    pub async fn run(self) {
        let container = self.container;
        let router = get_router(container);
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr).await.unwrap();

        axum::serve(listener, router).await.unwrap();
    }
}
