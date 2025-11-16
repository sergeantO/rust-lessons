pub mod command;
pub mod query;

#[cfg(test)]
mod tests {
    use dashmap::DashMap;
    use std::sync::Arc;

    use crate::{
        adapters::in_memory_repository::InMemoryRepository,
        app::{
            command::create_short_url::CreateShortUrlCommand, query::get_full_url::GetFullUrlQuery,
        },
        id_provider::NanoIdProvider,
    };

    #[tokio::test]
    async fn create_and_get_short_url() {
        // given
        let idp = NanoIdProvider;
        let store: Arc<DashMap<String, String>> = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store.clone());

        let create_command = CreateShortUrlCommand::new(idp, repo.clone());
        let get_query = GetFullUrlQuery::new(repo);

        // when
        let res = create_command
            .execute("https://google.com".to_owned())
            .await
            .unwrap();
        let res2 = get_query.execute(&res).await.unwrap();

        // then
        assert_eq!(res2, "https://google.com".to_owned())
    }
}
