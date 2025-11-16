pub trait GetFullUrlRepository {
    fn get(&self, short_url: &str) -> Result<String, String>;
}
pub struct GetFullUrlQuery<R>
where
    R: GetFullUrlRepository,
{
    repo: R,
}

impl<R> GetFullUrlQuery<R>
where
    R: GetFullUrlRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, short_url: &str) -> Result<String, String> {
        self.repo.get(short_url)
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use dashmap::DashMap;
    use tokio::join;

    use crate::adapters::in_memory_repository::InMemoryRepository;

    use super::*;

    #[tokio::test]
    async fn get_full_url() {
        // given
        struct FakeRepository;
        impl GetFullUrlRepository for FakeRepository {
            fn get(&self, _short_url: &str) -> Result<String, String> {
                Ok("123".to_owned())
            }
        }
        let repo = FakeRepository;
        let query = GetFullUrlQuery::new(repo);

        // when
        let result = query.execute("123").await;

        // then
        assert_eq!(result, Ok("123".to_owned()))
    }

    #[tokio::test]
    async fn get_full_url_grom_inmemory_repo() {
        // given
        let store: Arc<DashMap<String, String>> = Arc::new(DashMap::new());
        store.insert("123".to_owned(), "https://google.com".to_owned());

        let repo = InMemoryRepository::new(store);
        let query = GetFullUrlQuery::new(repo);

        // when
        let result = query.execute("123").await;

        // then
        assert_eq!(result, Ok("https://google.com".to_owned()));
    }

    #[tokio::test]
    async fn get_two_diferent_full_url() {
        // given
        let store: Arc<DashMap<String, String>> = Arc::new(DashMap::new());
        store.insert("123".to_owned(), "https://google.com".to_owned());
        store.insert("456".to_owned(), "https://github.com".to_owned());

        let repo = InMemoryRepository::new(store);
        let query = GetFullUrlQuery::new(repo);

        // when
        let result1 = query.execute("123");
        let result2 = query.execute("456");

        let (result1, result2) = join!(result1, result2);

        // then
        assert_eq!(result1, Ok("https://google.com".to_owned()));
        assert_eq!(result2, Ok("https://github.com".to_owned()));
    }
}
