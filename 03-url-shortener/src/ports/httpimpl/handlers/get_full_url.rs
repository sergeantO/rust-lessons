use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    app::{
        command::create_short_url::CreateShortUrlRepository,
        query::get_full_url::GetFullUrlRepository,
    },
    di::Container,
    id_provider::IDProvider,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FullUrlResponse {
    url: String,
}

impl From<String> for FullUrlResponse {
    fn from(url: String) -> Self {
        FullUrlResponse { url }
    }
}

/// ручка для получения полного url
pub async fn get_full_url<I, R, Q>(
    Path(id): Path<String>,
    State(container): State<Arc<Container<I, R, Q>>>,
) -> Result<Json<FullUrlResponse>, String>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    container
        .get_full_url_query
        .execute(&id)
        .await
        .map(|url| Json(FullUrlResponse::from(url)))
}
