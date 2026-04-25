pub mod api;
pub mod authentication;
pub mod config;
pub mod error;
pub mod model;

use mongodb::{
    Client, Collection,
    options::{ClientOptions, Compressor},
};

use crate::{
    config::CONFIG,
    model::{post::Post, user::User},
};

pub const DATABASE_NAME: &str = "skrunkly_draw";
pub const POST_COLLECTION_NAME: &str = "posts";
pub const USER_COLLECTION_NAME: &str = "users";

pub struct State {
    posts: Collection<Post>,
    users: Collection<User>,
}

impl State {
    /// Creates mongodb client
    ///
    /// # Panics
    /// Panics if the url is malformed or if the mongodb client can't be created.
    pub async fn new() -> Self {
        log::info!("Creating mongodb client...");

        let mut client_options = ClientOptions::parse(&CONFIG.database.url)
            .await
            .expect("Could not parse database.url");
        client_options.compressors = Some(vec![Compressor::Snappy]);

        let client = Client::with_options(client_options).expect("Could not create mongodb client");

        Self {
            posts: client
                .database(DATABASE_NAME)
                .collection(POST_COLLECTION_NAME),
            users: client
                .database(DATABASE_NAME)
                .collection(USER_COLLECTION_NAME),
        }
    }
}
