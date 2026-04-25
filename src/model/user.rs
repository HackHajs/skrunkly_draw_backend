use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    id: Uuid,
    created_at: DateTime<Utc>,
    name: String,
    link: String,
    profile_picture: Uuid,
}
