use crate::id_provider::IDProvider;

pub trait CreateShortUrlRepository {
    fn save(&self, full_url: String, short_url: String) -> Result<(), String>;
}

pub struct CreateShortUrlCommand<I, R>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
{
    id_provider: I,
    repo: R,
}

impl<I, R> CreateShortUrlCommand<I, R>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
{
    pub fn new(id_provider: I, repo: R) -> Self {
        Self { id_provider, repo }
    }

    pub async fn execute(&self, full_url: String) -> Result<String, String> {
        let id = self.id_provider.provide();
        self.repo.save(full_url, id.clone())?;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use dashmap::DashMap;

    use crate::{
        adapters::in_memory_repository::InMemoryRepository,
        id_provider::{FakeIDProvider, NanoIdProvider},
    };

    use super::*;

    #[tokio::test]
    async fn get_short_url() {
        // Given
        let idp = FakeIDProvider::new("123".to_owned());
        let store: Arc<DashMap<String, String>> = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store);
        let command = CreateShortUrlCommand::new(idp, repo);

        // when
        let result = command.execute("test".to_owned()).await;

        // then
        assert_ne!(result, Ok("".to_owned()))
    }

    #[tokio::test]
    async fn get_two_diferent_short_url() {
        // Given
        let idp = NanoIdProvider;
        let store: Arc<DashMap<String, String>> = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store);
        let command = CreateShortUrlCommand::new(idp, repo);

        // when
        let result1 = command.execute("test".to_owned()).await;
        let result2 = command.execute("test".to_owned()).await;

        // then
        assert_ne!(result1, result2)
    }

    #[tokio::test]
    async fn after_save_store_should_have_one_item() {
        let idp = NanoIdProvider;
        let store: Arc<DashMap<String, String>> = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store.clone());
        let command = CreateShortUrlCommand::new(idp, repo);

        // when
        let short_url = command.execute("test".to_owned()).await.unwrap();

        // then
        assert_eq!(store.len(), 1);
        let full_url = store.get(&short_url).unwrap();
        assert_eq!(full_url.value(), "test");
    }
}
