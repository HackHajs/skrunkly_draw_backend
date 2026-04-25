pub mod authentication;
pub mod config;
pub mod error;
pub mod model;

pub struct State;

impl State {
    pub async fn new() -> Self {
        Self
    }
}
