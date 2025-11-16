use crate::app::query::get_hello_world::{GetHelloWorld, Repository};

pub struct Container<R>
where
    R: Repository,
{
    pub hello_world_query: GetHelloWorld<R>,
}

impl<R> Container<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self {
            hello_world_query: GetHelloWorld::new(repository),
        }
    }
}
