use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    app::{
        command::create_short_url::CreateShortUrlRepository,
        query::get_full_url::GetFullUrlRepository,
    },
    di::Container,
    id_provider::IDProvider,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateShortUrlRequest {
    url: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ShortUrlResponse {
    url: String,
}

/// ручка для получения короткой ссылки
pub async fn shorten_url<I, R, Q>(
    State(container): State<Arc<Container<I, R, Q>>>,
    Json(input): Json<CreateShortUrlRequest>,
) -> Result<Json<ShortUrlResponse>, String>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    container
        .shorten_command
        .execute(input.url)
        .await
        .map(|id| Json(ShortUrlResponse { url: id }))
}
