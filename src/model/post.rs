use chrono::{DateTime, Utc};
use futures_util::TryStreamExt;
use mongodb::{Collection, bson::doc};
use uuid::Uuid;

use exn::ResultExt;

use crate::{error::Error, model::skrunkle::Skrunkle};

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct Post {
    #[serde(rename = "_id")]
    id: Uuid,
    user: Option<Uuid>,
    created_at: Option<DateTime<Utc>>,
    reply: Option<Reply>,
    mature: bool,
    liked_by: Option<Vec<Uuid>>,
    flagged_by: Option<Vec<Uuid>>,
    skrunkle: Skrunkle,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Debug, Clone)]
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
        self.user = Some(user);
        self.created_at = Some(Utc::now());
        self.reply = None;
        self.liked_by = None;
        self.liked_by = None;

        collection
            .insert_one(self)
            .await
            .or_raise(|| Error::upstream("Failed to insert post".into()))?;

        Ok(())
    }

    pub async fn get(post_collection: &Collection<Post>) -> exn::Result<Vec<Post>, Error> {
        let mut posts = post_collection
            .find(doc! {})
            .await
            .or_raise(|| Error::upstream("Failed to fetch posts".into()))?;

        let mut miau: Vec<Post> = Vec::new();

        while let Some(post) = posts
            .try_next()
            .await
            .or_raise(|| Error::upstream("Failed to fetch a post".into()))?
        {
            miau.push(post);
        }

        Ok(miau)
    }
}
