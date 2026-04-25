pub mod api;
pub mod authentication;
pub mod config;
pub mod error;

use mongodb::{
    Client,
    options::{ClientOptions, Compressor},
};

use crate::config::CONFIG;

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
