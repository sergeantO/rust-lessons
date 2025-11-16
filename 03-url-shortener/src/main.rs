use dashmap::DashMap;
use std::sync::Arc;

use crate::ports::httpimpl::server::Server;

pub mod adapters;
pub mod app;
pub mod di;
pub mod id_provider;
pub mod ports;

#[tokio::main]
async fn main() {
    let store = Arc::new(DashMap::new());
    let in_mem = adapters::in_memory_repository::InMemoryRepository::new(store);
    let idp = id_provider::NanoIdProvider;
    let container = Arc::new(di::Container::new(idp, in_mem.clone(), in_mem));

    let server = Server::new(3001, container);
    server.run().await;
}
