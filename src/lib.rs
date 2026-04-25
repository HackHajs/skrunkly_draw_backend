pub mod api;
pub mod authentication;
pub mod config;
pub mod error;
pub mod model;

use mongodb::{
    Client,
    options::{ClientOptions, Compressor},
};

use crate::config::CONFIG;

pub const DATABASE_NAME: &str = "skrunkly_draw";
pub const POST_COLLECTION_NAME: &str = "posts";

pub struct State(pub Client);

impl State {
    pub async fn new() -> Self {
        log::info!("Creating mongodb client...");

        let mut client_options = ClientOptions::parse(&CONFIG.database.url)
            .await
            .expect("Could not parse database.url");
        client_options.compressors = Some(vec![Compressor::Snappy]);

        Self(Client::with_options(client_options).expect("Could not create mongodb client"))
    }
}
