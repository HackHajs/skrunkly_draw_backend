use chrono::{DateTime, Utc};
use futures_util::TryStreamExt;
use mongodb::{
    Collection,
    bson::{doc, uuid::Uuid},
};

use exn::ResultExt;

use crate::{error::Error, model::skrunkle::Skrunkle};

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct Post {
    #[serde(rename = "_id")]
    id: Option<Uuid>,
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
    /// Will return an error if the post object is malformed.
    /// Might return an error if there's an issue communicating with the database.
    pub async fn insert(
        mut self,
        collection: &Collection<Self>,
        user: Uuid,
    ) -> exn::Result<(), Error> {
        self.id = Some(Uuid::new());
        self.user = Some(user);
        self.created_at = Some(Utc::now());
        self.liked_by = None;
        self.flagged_by = None;

        collection
            .insert_one(self)
            .await
            .map_err(Error::from)
            .or_raise(|| Error::upstream("Failed to insert post".into()))?;

        Ok(())
    }

    /// Add the post to the database
    ///
    /// # Errors
    /// Might return an error if there's an issue communicating with the database.
    pub async fn get_all(collection: &Collection<Self>) -> exn::Result<Vec<Self>, Error> {

        let mut posts = collection
            .find(doc! {})
            .sort(doc! { "created_at": -1 })
            .await
            .map_err(Error::from)
            .or_raise(|| Error::upstream("Failed to fetch posts".into()))?;


        let mut post_list: Vec<Self> = Vec::new();

        while let Some(post) = posts
            .try_next()
            .await
            .map_err(Error::from)
            .or_raise(|| Error::upstream("Failed to fetch a post".into()))?
        {
            post_list.push(post);
        }

        Ok(post_list)
    }

    /// Remove the post from the database
    ///
    /// Returns the number of updated documents.
    ///
    /// # Errors
    /// Might return an error if there's an issue communicating with the database.
    pub async fn delete(
        collection: &Collection<Self>,
        post: Uuid,
        user: Uuid,
    ) -> exn::Result<u64, Error> {
        Ok(collection
            .delete_one(doc! {
                "$and": [
                    doc! { "_id": post },
                    doc! { "user": user }
                ]
            })
            .await
            .map_err(Error::from)
            .or_raise(|| Error::upstream("Failed to delete post".into()))?
            .deleted_count)
    }
}
