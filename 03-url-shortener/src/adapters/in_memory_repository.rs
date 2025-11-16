use std::sync::Arc;

use dashmap::DashMap;

use crate::app::{
    command::create_short_url::CreateShortUrlRepository, query::get_full_url::GetFullUrlRepository,
};

#[derive(Clone)]
pub struct InMemoryRepository {
    store: Arc<DashMap<String, String>>,
}

impl InMemoryRepository {
    pub fn new(store: Arc<DashMap<String, String>>) -> Self {
        Self { store }
    }
}

impl CreateShortUrlRepository for InMemoryRepository {
    fn save(&self, full_url: String, short_url: String) -> Result<(), String> {
        self.store.insert(short_url, full_url);

        Ok(())
    }
}

impl GetFullUrlRepository for InMemoryRepository {
    fn get(&self, short_url: &str) -> Result<String, String> {
        let res = self.store.get(short_url);
        match res {
            Some(full_url) => Ok(full_url.clone()),
            None => Err("Not Found".to_owned()),
        }
    }
}
