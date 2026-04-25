use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::skrunkle::Skrunkle;

#[derive(serde::Serialize, serde::Deserialize)]
struct Post {
    #[serde(rename = "_id")]
    id: Uuid,
    created_at: DateTime<Utc>,
    reply: Reply,
    mature: bool,
    liked_by: Vec<Uuid>,
    flagged_by: Vec<Uuid>,
    skrunkle: Skrunkle,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Reply {
    parent: Uuid,
    on_feed: bool,
}
