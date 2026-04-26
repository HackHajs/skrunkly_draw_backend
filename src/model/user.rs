use chrono::{DateTime, Utc};
use mongodb::{
    Collection,
    bson::{Uuid, doc},
    options::UpdateModifications,
};

use exn::ResultExt;

use crate::error::{DatabaseError as DbErr, Error};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    id: Option<Uuid>,
    created_at: Option<DateTime<Utc>>,
    name: Option<String>,
    link: Option<String>,
    profile_picture: Option<Uuid>,
}

impl From<User> for UpdateModifications {
    fn from(val: User) -> Self {
        let mut changes = doc! {};

        if let Some(name) = val.name {
            _ = changes.insert("name", name);
        }

        if let Some(link) = val.link {
            _ = changes.insert("link", link);
        }

        if let Some(pfp) = val.profile_picture {
            _ = changes.insert("profile_picture", pfp);
        }

        let mut doc = doc! {};
        doc.insert("$set", changes);

        Self::Document(doc)
    }
}

impl User {
    /// Add the user to the database, or update it if already exists
    ///
    /// # Errors
    /// Will return an error if the user object is malformed.
    /// Might return an error if there's an issue communicating with the database.
    pub async fn upsert(
        mut self,
        collection: &Collection<Self>,
        user: Uuid,
    ) -> exn::Result<(), Error> {
        // Insert
        self.id = Some(user);
        self.created_at = Some(Utc::now());

        collection
            .update_one(doc! {"_id": user}, self)
            .upsert(true)
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
    pub async fn get(user_collection: &Collection<Self>, user: Uuid) -> exn::Result<Self, Error> {
        let filter = doc! { "_id" : user };
        let user = user_collection
            .find_one(filter)
            .await
            .map_err(Error::from)
            .or_raise(|| Error::upstream("Failed to fetch user".into()))?;

        match user {
            Some(u) => Ok(u),
            None => Err(Error::database(DbErr::NotFound, "User not found".into()))?,
        }
    }

    /// Remove the user from the database
    ///
    /// Returns the number of updated documents.
    ///
    /// # Errors
    /// Might return an error if there's an issue communicating with the database.
    pub async fn delete(collection: &Collection<Self>, user: Uuid) -> exn::Result<u64, Error> {
        Ok(collection
            .delete_one(doc! {
                "_id": user
            })
            .await
            .map_err(Error::from)
            .or_raise(|| Error::upstream("Failed to delete user".into()))?
            .deleted_count)
    }
}
