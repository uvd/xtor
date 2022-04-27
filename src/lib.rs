use std::collections::HashMap;

use actor::actor::ACTOR_ID_HANDLE;
use futures::{future::join_all, Future};

#[cfg(test)]
mod tests;

pub use xtor_derive::main;
pub use xtor_derive::message;

pub mod actor;

#[inline(always)]
pub async fn await_exit() {
    let mut actors = ACTOR_ID_HANDLE.write().await;
    let actors = std::mem::replace(&mut *actors, HashMap::new());
    let actors = actors.into_values().collect::<Vec<_>>();
    join_all(actors).await;
}

pub fn block_on<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(future)
}
