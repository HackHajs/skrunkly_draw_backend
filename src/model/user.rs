use chrono::{DateTime, Utc};
use mongodb::{
    Collection,
    bson::{Uuid, doc},
};

use exn::ResultExt;

use crate::error::Error;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    id: Option<Uuid>,
    created_at: Option<DateTime<Utc>>,
    name: String,
    link: Option<String>,
    profile_picture: Option<Uuid>,
}

impl User {
    /// Add the user to the database
    ///
    /// # Errors
    /// Will return an error if the user object is malformed.
    /// Might return an error if there's an issue communicating with the database.
    pub async fn insert(
        mut self,
        collection: &Collection<Self>,
        user: Uuid,
    ) -> exn::Result<(), Error> {
        self.id = Some(user);
        self.created_at = Some(Utc::now());

        collection
            .insert_one(self)
            .await
            .map_err(Error::from)
            .or_raise(|| Error::upstream("Failed to insert user".into()))?;
        Ok(())
    }

    /// Get a user from the database
    ///
    /// # Errors
    /// Will return an error if the user object is malformed.
    /// Might return an error if there's an issue communicating with the database.
    pub async fn get(id: Uuid, user_collection: &Collection<Self>) -> exn::Result<Self, Error> {
        let filter = doc! { "_id" : id };
        let user = user_collection
            .find_one(filter)
            .await
            .map_err(Error::from)
            .or_raise(|| Error::upstream("Failed to fetch user".into()))?;

        match user {
            Some(u) => Ok(u),
            None => Err(Error::upstream("Failed to find user".into()))?,
        }
    }
}
