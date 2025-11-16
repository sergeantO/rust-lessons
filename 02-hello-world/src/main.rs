use std::sync::Arc;

use crate::{app::query::get_hello_world::InMemoryRepo, di::Container};

pub mod app;
pub mod di;
pub mod ports;

#[tokio::main]
async fn main() {
    let repo = InMemoryRepo;
    let container = Arc::new(Container::new(repo));
    let server = ports::httpapi::Server::new(3001, container);

    server.run().await;
}
