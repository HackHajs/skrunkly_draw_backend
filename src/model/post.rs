use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use mongodb::{Collection, bson::doc};
use uuid::Uuid;

use exn::ResultExt;

use crate::{error::Error, model::skrunkle::Skrunkle};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Post {
    #[serde(rename = "_id", skip_deserializing)]
    id: Uuid,
    #[serde(skip_deserializing)]
    user: Uuid,
    #[serde(skip_deserializing)]
    created_at: DateTime<Utc>,
    reply: Option<Reply>,
    mature: bool,
    #[serde(skip_deserializing)]
    liked_by: Vec<Uuid>,
    #[serde(skip_deserializing)]
    flagged_by: Vec<Uuid>,
    skrunkle: Skrunkle,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct Reply {
    parent: Uuid,
    on_feed: bool,
}

impl Post {
    /// Add the post to the database
    ///
    /// # Errors
    /// Will return an error if the post object is malformed or if the user is unauthenticated.
    /// Might return an error if there's an issue communicating with the database.
    pub async fn insert(
        mut self,
        collection: &Collection<Self>,
        user: Uuid,
    ) -> exn::Result<(), Error> {
        self.id = uuid::Uuid::new_v4();
        self.user = user;
        self.created_at = Utc::now();

        collection
            .insert_one(self)
            .await
            .or_raise(|| Error::upstream("Failed to insert post".into()))?;

        Ok(())
    }

    pub async fn get(post_collection: &Collection<Post>) -> exn::Result<Vec<Post>, Error> {
        let posts = post_collection
            .find(doc! {})
            .await
            .or_raise(|| Error::upstream("Failed to fetch posts".into()))?;

        Result::Ok(todo!())
    }
}
