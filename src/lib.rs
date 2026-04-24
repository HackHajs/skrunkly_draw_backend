pub mod authentication;
pub mod config;
pub mod error;

pub struct State;

impl State {
    pub async fn new() -> Self {
        Self
    }
}
