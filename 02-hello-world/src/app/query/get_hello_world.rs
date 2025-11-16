pub trait Repository {
    fn get_hello_world(&self) -> impl Future<Output = &'static str> + Send;
}

pub struct GetHelloWorld<R>
where
    R: Repository,
{
    repo: R,
}

impl<R> GetHelloWorld<R>
where
    R: Repository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> &'static str {
        self.repo.get_hello_world().await
    }
}

#[derive(Default)]
pub struct InMemoryRepo;

impl Repository for InMemoryRepo {
    async fn get_hello_world(&self) -> &'static str {
        "Hello, world!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_hello() {
        let repo = InMemoryRepo;
        let query = GetHelloWorld::new(repo);

        // when
        let result = query.execute().await;

        // then
        assert_eq!(result, "Hello, world!");
    }
}
