use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use tokio::sync::mpsc;
use warp::ws::Message;
use std::sync::atomic::AtomicUsize;
use common::TokenNetwork;

pub static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);
pub type SharedState = Arc<Mutex<GameState>>;

#[derive(Debug)]
pub struct GameState {
    pub clients: HashMap<usize, mpsc::UnboundedSender<Message>>,
    pub tokens: HashMap<u32, TokenNetwork>,
}

pub fn with_state(state: SharedState)
  -> impl Filter<Extract = (SharedState,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || state.clone())
}
