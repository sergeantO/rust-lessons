use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    app::{
        command::create_short_url::CreateShortUrlRepository,
        query::get_full_url::GetFullUrlRepository,
    },
    di::Container,
    id_provider::IDProvider,
};

/// маппинг урлов
pub fn get_router<I, R, Q>(contaiter: Arc<Container<I, R, Q>>) -> Router
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    use crate::ports::httpimpl::handlers::get_full_url::get_full_url;
    use crate::ports::httpimpl::handlers::shorten_url::shorten_url;

    Router::new()
        .route("/{id}", get(get_full_url))
        .route("/", post(shorten_url))
        .with_state(contaiter)
}
